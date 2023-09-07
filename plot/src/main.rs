use plotters::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::Path;

type Table = HashMap<(String, String), Vec<(usize, f32)>>;

fn load_csv() -> (Table, Table, Table) {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() < 2 {
        panic!("Not enough arguments")
    }

    let path = Path::new(&argv[1]);

    let mut rdr = csv::Reader::from_path(path).unwrap();

    let mut index: HashMap<&str, usize> = HashMap::new();

    for (c, hdr) in rdr.headers().unwrap().into_iter().enumerate() {
        index.insert(hdr, c);
    }

    let mut rdr = csv::Reader::from_path(path).unwrap();

    let data: Vec<Vec<String>> = rdr
        .records()
        .filter_map(|row| if let Ok(row) = row { Some(row) } else { None })
        .map(|row| {
            let mut vec = Vec::new();
            for e in row.into_iter() {
                vec.push(String::from(e))
            }
            vec
        })
        .collect();

    let mut proof_durations = HashMap::new();
    let mut verify_durations = HashMap::new();
    let mut proof_sizes = HashMap::new();

    data.into_iter().for_each(|row| {
        let job_size = row[index["job_size"]].parse::<usize>().unwrap();
        let key = (row[index["job_name"]].clone(), row[index["prover"]].clone());
        let proof_duration = row[index["proof_duration_millisec"]]
            .parse::<f32>()
            .unwrap();
        let verify_duration = row[index["verify_duration_millisec"]]
            .parse::<f32>()
            .unwrap();
        let proof_size = row[index["proof_bytes"]].parse::<f32>().unwrap();

        // cast milliseconds to seconds for proof duration
        proof_durations
            .entry(key.clone())
            .or_insert(Vec::new())
            .push((job_size, proof_duration / 1_000.0));
        verify_durations
            .entry(key.clone())
            .or_insert(Vec::new())
            .push((job_size, verify_duration));
        proof_sizes
            .entry(key)
            .or_insert(Vec::new())
            .push((job_size, proof_size));
    });

    proof_durations
        .values_mut()
        .for_each(|v| v.sort_by(|(s1, _), (s2, _)| s1.cmp(s2)));

    (proof_durations, verify_durations, proof_sizes)
}

const PROVERS: [&str; 3] = ["Bonsai", "CpuSHA256", "CpuPoseidon"];

fn plot_data(filename: &str, title: &str, data: &Vec<Vec<(usize, f32)>>) {
    let root_area = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let root_area = root_area.titled(title, ("sans-serif", 60)).unwrap();

    let (xmin, xmax) = data
        .iter()
        .flatten()
        .map(|(x, _y)| *x as f32)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), x| {
            (min.min(x), max.max(x))
        });

    let (_ymin, ymax) = data
        .iter()
        .flatten()
        .map(|(_x, y)| *y)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), y| {
            (min.min(y), max.max(y))
        });

    let xrange = xmin..xmax;
    let yrange = 0.0..(ymax * 1.5);

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        // .caption("Sine and Cosine", ("sans-serif", 40))
        .build_cartesian_2d(xrange, yrange)
        .unwrap();

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()
        .unwrap();

    data.iter().enumerate().for_each(|(index, v)| {
        let prover = PROVERS[index];
        let points = v.iter().map(|(s, x)| (*s as f32, *x));
        let series = LineSeries::new(points, &Palette99::pick(index));
        cc.draw_series(series)
            .unwrap()
            .label(prover)
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(index))
            });
    });

    cc.configure_series_labels()
        .border_style(BLACK)
        .draw()
        .unwrap();

    // data.iter().for_each(|v| {
    //     let points = v.iter().map(|(s, x)| (*s as f32, *x));
    //     let series = LineSeries::new(points, &RED);
    //     cc.draw_series(series).unwrap();
    // });

    root_area.present().expect("Unable to write result to file");
}

fn display() {
    // Plotting
    let (proof_durations, verify_durations, proof_sizes) = load_csv();

    let all_data = [
        (&proof_durations, "proving_time"),
        (&verify_durations, "verifying_time"),
        (&proof_sizes, "proof_size"),
    ];

    for (hmap, name) in all_data {
        let mut jobs = HashSet::new();
        hmap.keys().for_each(|(job_name, _)| {
            let _ = jobs.insert(job_name);
        });

        for j in jobs {
            // proving time
            let mut data = Vec::new();

            for prover in PROVERS {
                let k = &(j.clone(), String::from(prover));
                data.push(hmap[k].clone());
            }

            let filename = format!("{}_{}.png", name, j);
            let title = format!("{} {}", name, j);

            plot_data(&filename, &title, &data)
        }
    }

    // for prover in PROVERS {
    //     let data = jobs.iter ().map (|job|
    //                                  {
    //                                      let k = (job_name, String::from(prover));
    //                                      let data =
    //                                  }
    //     )
    // }

    // proof_durations.into_iter().for_each(|(k, v)| {
    //     let (job_name, prover) = k;

    //     let filename = format!("proving_time_{}_{}.png", job_name, prover);
    //     let title = format!("proving time {} {}", job_name, prover);

    //     plot_data(&filename, &title, &v)
    // });

    // verify_durations.into_iter().for_each(|(k, v)| {
    //     let (job_name, prover) = k;

    //     let filename = format!("verifying_time_{}_{}.png", job_name, prover);
    //     let title = format!("verifying time {} {}", job_name, prover);

    //     plot_data(&filename, &title, &v)
    // });

    // proof_sizes.into_iter().for_each(|(k, v)| {
    //     let (job_name, prover) = k;

    //     let filename = format!("proof_size_{}_{}.png", job_name, prover);
    //     let title = format!("proof size {} {}", job_name, prover);

    //     plot_data(&filename, &title, &v)
    // });
}

fn main() {
    display()
}
