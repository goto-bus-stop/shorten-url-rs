use criterion::*;
use shorten_url::shorten;

fn try_a_bunch() {
    shorten(
        "https://www.vpro.nl/programmas/gliphoeve/documentaire-intro.html",
        50,
    );
    shorten(
        "https://discordapp.com/channels/317475976369930241/317475976369930241",
        25,
    );
    shorten(
        "http://example.com/ultra/cool/page/that-is-really-deeply/nested/",
        30,
    );
    // shorten("http://www.blahblah.com/unpragmatic-thoughts/?p=1738", 50);
    // shorten("https://www.reddit.com/?count=25&after=t3_76zjp1", 40);
    // shorten("https://www.thisisasuperlonghostname.co.uk", 35);
    // shorten("https://www.thisisasuperlonghostname.co.uk", 35);
    // shorten("::::::::::::::::::::::::::::::::::::::::::::::::::::::::ƽ:::::::::::::::!:::::::::::::::::::::::::::::::::::::::::::::::::::::\u{0}:\u{6}::::::::::::", 58);
    // shorten("::::/::::::::::::2::::::::::::::::?:ƽ:%%*", 37);
}

fn bench(c: &mut Criterion) {
    c.bench_function("shorten", |b| {
        b.iter(|| try_a_bunch());
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
