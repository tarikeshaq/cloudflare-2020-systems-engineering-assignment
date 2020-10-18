use crate::http::HttpRequest;
use ansi_term::Color::Green;

struct ResponseData {
    time: std::time::Duration,
    bytes_read: usize,
    err_code: Option<u32>,
}

pub fn run_profile(http_request: &HttpRequest, num_of_requests: &str) -> anyhow::Result<()> {
    let num_of_requests = num_of_requests.parse::<u32>()?;

    let mut data = vec![];
    for _ in 0..num_of_requests {
        let before = std::time::Instant::now();
        let (read, mut buff) = http_request.run()?;
        let time = before.elapsed();
        data.push(ResponseData {
            time,
            bytes_read: read,
            err_code: is_error(&mut buff),
        });
    }
    print_profile(num_of_requests, &data);
    Ok(())
}

fn print_profile(num_of_requests: u32, data: &[ResponseData]) {
    let mut durations = data
        .iter()
        .map(|rd| rd.time)
        .collect::<Vec<std::time::Duration>>();
    durations.sort();
    let fastest_time = durations.iter().min().unwrap();
    let slowest_time = durations.iter().max().unwrap();
    let sum: std::time::Duration = durations.iter().sum();
    let mean = sum.as_millis() as f64 / num_of_requests as f64;
    let mid = num_of_requests / 2;
    let median = durations[mid as usize];
    let success = data.iter().filter(|rd| rd.err_code.is_none()).count();
    let sucess_percent = success as f64 / num_of_requests as f64;
    let err_codes = data
        .iter()
        .filter_map(|rd| rd.err_code)
        .collect::<Vec<u32>>();
    let num_bytes = data.iter().map(|rd| rd.bytes_read).collect::<Vec<usize>>();
    let max_read = num_bytes.iter().max().unwrap();
    let min_read = num_bytes.iter().min().unwrap();

    let display_str = format!("Number of Requests:   {}\nFastest Time:    {}ms\nSlowest Time:    {}ms\nMean Time:    {}ms\nMedian Time:   {}ms\nSuccessful Percentage:   {}%\nError Codes:   {:?}\nSmallest Size:   {} bytes\nLargest Size:   {}bytes",
                                        num_of_requests, fastest_time.as_millis(), slowest_time.as_millis(), mean, median.as_millis(), sucess_percent * 100f64, err_codes, min_read, max_read);

    billboard::Billboard::default().display(Green.paint(&display_str).to_string().as_ref());
}

fn is_error(buff: &mut [u8]) -> Option<u32> {
    let buff = std::str::from_utf8(buff).unwrap();
    let mut lines = buff.lines();
    let mut split = lines.next().unwrap().split_ascii_whitespace();
    split.next();
    let code = split.next().unwrap();
    let code = code.parse::<u32>().unwrap();
    if code != 200 {
        Some(code)
    } else {
        None
    }
}
