use event_iterator::{EventIterator, LendAs};

#[derive(Copy, Clone)]
struct Uwu<'a>(&'a i32);

#[derive(Copy, Clone)]
struct LendAsUwu;

impl LendAs for LendAsUwu {
    type From<'a> = &'a i32;
    type Into<'a> = Uwu<'a>;

    fn lend_as<'a>(self, from: Self::From<'a>) -> Self::Into<'a> {
        Uwu(from)
    }
}

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut ei = event_iterator::from_iter_ref([1, 2, 3, 4, 5]).map(LendAsUwu);

    while let Some(Uwu(i)) = ei.next().await {
        println!("{i}");
    }
}
