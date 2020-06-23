use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bench_thomas_vs_elrendio::Message;
/* fn thomas(messages: &[Message]) -> Option<&Message> {
    let search_from = messages
        .iter()
        .rposition(|message| match message {
            Message::Received { is_flagged: true } | Message::Sent { is_manual: true } => true,
            _ => false,
        })
        .map_or(0, |i| i + 1);
    // find first unhandled message and check its sent_at date to trigger the auto msg
    messages
        .iter()
        .skip(search_from)
        .find(|message| match message {
            Message::Received { is_flagged: false } => true,
            _ => false,
        })
} */

// truly oscar method
fn thomas(messages: &[Message]) -> Option<&Message> {
    let mut unhandled_received_message = None;
    messages
        .iter()
        .inspect(|&message| match message {
            Message::Received { is_flagged: false } => {
                unhandled_received_message = Some(message);
            }
            _ => (),
        })
        .rposition(|message| match message {
            Message::Received { is_flagged: true } | Message::Sent { is_manual: true } => true,
            _ => false,
        });
    unhandled_received_message
}
fn elrendio(messages: &[Message]) -> Option<&Message> {
    let mut unhandled_received_message = None;
    for message in messages.iter().rev() {
        match message {
            Message::Sent { is_manual: true } => break,
            Message::Sent { is_manual: false } => {}
            Message::Received { is_flagged: true } => break,
            Message::Received { is_flagged: false } => {
                unhandled_received_message = Some(message);
            }
        }
    }

    unhandled_received_message
}
fn criterion_benchmark(c: &mut Criterion) {
    let history = bench_thomas_vs_elrendio::history();
    let history_happy = bench_thomas_vs_elrendio::history_happy();
    let history_happy2 = bench_thomas_vs_elrendio::history_happy2();
    let history_happy3 = bench_thomas_vs_elrendio::history_happy3();
    c.bench_function("thomas function", |b| {
        b.iter(|| thomas(black_box(&history)))
    });
    c.bench_function("thomas function happy", |b| {
        b.iter(|| thomas(black_box(&history_happy)))
    });
    c.bench_function("thomas function happy 2", |b| {
        b.iter(|| thomas(black_box(&history_happy2)))
    });
    c.bench_function("thomas function happy 3", |b| {
        b.iter(|| thomas(black_box(&history_happy3)))
    });
    c.bench_function("elrendio function", |b| {
        b.iter(|| elrendio(black_box(&history)))
    });
    c.bench_function("elrendio function happy", |b| {
        b.iter(|| elrendio(black_box(&history_happy)))
    });
    c.bench_function("elrendio function happy 2", |b| {
        b.iter(|| elrendio(black_box(&history_happy2)))
    });
    c.bench_function("elrendio function happy 3", |b| {
        b.iter(|| elrendio(black_box(&history_happy3)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
