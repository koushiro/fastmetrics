use fastmetrics_derive::LabelSet;

#[derive(LabelSet)]
struct Labels {
    status: HttpStatus,
}

struct HttpStatus(u16);

fn main() {}
