use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use nexus::{
    poll::source::yougov::api::{get::get_latest_editorial_url, parse::parse_pages},
    utils::pdf::extract::extract_pdf_from_url,
};

fn yougov_page_parsing(c: &mut Criterion) {
    let url = get_latest_editorial_url();
    let pages = extract_pdf_from_url(url.as_str());
    assert!(
        !pages.is_empty(),
        "latest YouGov editorial PDF had no pages"
    );

    let mut group = c.benchmark_group("yougov_page_parsing");
    group.bench_with_input(
        BenchmarkId::new("parallel_pages_serial_split", pages.len()),
        &pages,
        |b, pages| {
            b.iter(|| parse_pages(black_box(pages)));
        },
    );
    group.finish();
}

criterion_group!(benches, yougov_page_parsing);
criterion_main!(benches);
