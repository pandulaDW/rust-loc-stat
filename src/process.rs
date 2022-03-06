use super::config::{Config, Language};
use super::parser::{LineStats, Parser};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, fs::File, io, thread};

use crossbeam::channel::{bounded, Receiver, Sender};
use crossbeam::sync::WaitGroup;

/// Helper struct to pass the file_paths around
struct HandlerWithLanguage {
    path: PathBuf,
    language: Language,
}

/// Parses the files concurrently using an un-buffered channel by first utilizing a single-consumer-multiple-producer
/// approach and then aggregating the results using a multiple-producer-single-consumer pattern.
///
/// Data races are not possible since aggregation is done by only one thread. This is a much more performant approach
/// compared to using Locks to synchronize memory access.
///
/// Since, the standard library only allows mpsc channels, `crossbeam` library is used here to achieve the channel
/// behaviors as it offers more features and better performance compared to the standard library.
pub fn process_files(conf: &Config) -> io::Result<()> {
    // unbuffered channels which handles the fan-out and fan-in patterns
    let (file_producer, parser_consumer) = bounded(0);
    let (parser_producer, aggregate_consumer) = bounded(0);

    // spawning parser threads
    for _ in 0..3 {
        spawn_worker_thread_for_parsing(parser_consumer.clone(), parser_producer.clone());
    }

    // dropping the original channels to avoid having the program hang on receiving or sending
    drop(parser_consumer);
    drop(parser_producer);

    // creating a WaitGroup to wait until the aggregation is finished
    let wg = WaitGroup::new();
    let w_for_agg = wg.clone();

    // spawning a single consumer to aggregate the results.
    // This will print out the end results
    thread::spawn(move || {
        let mut results_map = initiate_results_map();

        loop {
            if let Ok(parser) = aggregate_consumer.recv() {
                aggregate_results(&mut results_map, parser);
            } else {
                println!("{:?}", results_map);
                drop(w_for_agg);
                break;
            }
        }
    });

    // collect the file paths with the correct languages
    let mut handlers = Vec::new();
    read_files(&conf.directory, &conf.excluded_dirs, &mut handlers)?;

    // send each file to available consumers
    for h in handlers {
        file_producer.send(h).unwrap();
    }

    // drop the file producer after handling all the files. Which would drop the parser consumers.
    drop(file_producer);

    // block until aggregation thread has finished its work.
    wg.wait();

    Ok(())
}

/// Walks the directory recursively, open each file handler found and process it in a sequence.
///
/// Handlers will be dropped one after the other, to avoid panicking after having too many open
/// file handlers.
///
/// If there's an error in handling a file, that file will be omitted silently.
fn read_files(
    dir: &Path,
    excluded_dirs: &Vec<PathBuf>,
    handlers: &mut Vec<HandlerWithLanguage>,
) -> io::Result<()> {
    if dir.is_dir() {
        if is_dir_excluded(dir, excluded_dirs) {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                read_files(&path, excluded_dirs, handlers)?;
            } else {
                if let Some(language) = get_language_by_extension(&path) {
                    handlers.push(HandlerWithLanguage { path, language })
                }
            }
        }
    }

    Ok(())
}

// process the individual file. Parses and then aggregate the result
fn process_file(handler: HandlerWithLanguage) -> io::Result<Parser> {
    // create the buffered reader
    let f = File::open(handler.path)?;
    let buf_reader = io::BufReader::new(f);

    // parse the file
    let mut parser = Parser::new(handler.language);
    parser.parse(buf_reader)?;

    // // aggregate the result
    // self.aggregate_result(parser);

    Ok(parser)
}

// spawn a thread to consume files and produce results
fn spawn_worker_thread_for_parsing(
    consumer_ch: Receiver<HandlerWithLanguage>,
    prod_ch: Sender<Parser>,
) {
    thread::spawn(move || loop {
        if let Ok(handler) = consumer_ch.recv() {
            let p = process_file(handler).unwrap();
            prod_ch.send(p).unwrap();
        } else {
            break;
        }
    });
}

// Insert an entry for each support language
fn initiate_results_map() -> HashMap<Language, LineStats> {
    let mut map = HashMap::new();
    map.insert(Language::Javascript, (0, 0, 0));
    map.insert(Language::Typescript, (0, 0, 0));
    return map;
}

// aggregate the results for each language
fn aggregate_results(result_map: &mut HashMap<Language, LineStats>, parser: Parser) {
    let mut language_counts = result_map[&parser.language];
    language_counts.0 += parser.line_stats.0;
    language_counts.1 += parser.line_stats.1;
    language_counts.2 += parser.line_stats.2;
    result_map.insert(parser.language, language_counts);
}

// check if given directory is an excluded directory
fn is_dir_excluded(dir: &Path, excluded_dirs: &Vec<PathBuf>) -> bool {
    for path in excluded_dirs.iter() {
        if dir.starts_with(path) {
            return true;
        }
    }
    false
}

// get the language by looking at the file extension. If the extension is unknown, None will be returned
fn get_language_by_extension(path: &Path) -> Option<Language> {
    if let Some(extension) = path.extension() {
        let ext_str = extension.to_str().unwrap_or("");
        match ext_str {
            "js" | "jsx" => Some(Language::Javascript),
            "ts" | "tsx" => Some(Language::Typescript),
            _ => None,
        }
    } else {
        None
    }
}
