use criterion::{criterion_group, criterion_main, Criterion};
use deb::control::RawParagraph;

macro_rules! benchmark_paragraph {
    ($grp:ident, $name:ident, $para:expr) => {
        $grp.bench_function(stringify!($name), |b| {
            b.iter(|| {
                RawParagraph::parse($para).unwrap();
            })
        });
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("raw_paragraph_parse");

        benchmark_paragraph!(
            group,
            simple,
            "\
Hello: World
Hello1: World2
Single:
 Multi line
 Value goes
 Here
"
        );

        benchmark_paragraph!(
            group,
            simple_comment,
            "\
Hello: World
# Comment here
Hello1: World2
Single:
 Multi line
 Value goes
 Here
"
        );

        benchmark_paragraph!(
            group,
            longish,
            "\
Lorem-Ipsum:
 Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce commodo molestie mauris vel laoreet.
 Donec in magna placerat, laoreet augue ac, varius sem. Sed non facilisis dui, ac condimentum dolor.
 Donec eu sem non ante iaculis ornare et pharetra lorem. Nunc pretium ipsum id arcu venenatis
 facilisis. Nam iaculis purus vel lobortis tempus. Curabitur quis vulputate turpis. Aliquam eleifend
 pretium ante semper malesuada. Suspendisse at ipsum accumsan, accumsan lectus in, feugiat libero.
 Etiam eu luctus risus, in varius lorem. Cras pretium tortor sed ligula lobortis mollis.
 .
 Curabitur in augue aliquam, tempus turpis ut, gravida ex. Proin mattis hendrerit mauris sed
 lacinia. Donec urna est, lacinia non semper eleifend, egestas et arcu. Orci varius natoque
 penatibus et magnis dis parturient montes, nascetur ridiculous mus. Quisque erat sem, egestas vel
 bibendum sed, consectetur nec metus. Quisque vehicula eros quis augue consectetur varius. In
 bibendum lacinia diam eu pulvinar. Proin lobortis eros quam, at consequat turpis imperdiet at.
 Fusce a ipsum a sapien egestas vehicula. Suspendisse sit amet tempor velit, vehicula ullamcorper
 mi. Proin sed facilisis lectus, sit amet rutrum lectus. Ut at mauris eu lectus fringilla
 pellentesque. Aliquam gravida sollicitudin nisi non cursus. Nunc venenatis erat diam, eu convallis
 ante sollicitudin et.
 .
 Pellentesque quis ante tempor arcu congue mollis. Vivamus dapibus pharetra dapibus. Quisque vel
 porta elit. Proin ante lectus, convallis ultrices bibendum in, mattis sit amet enim. Aliquam
 venenatis non quam nec semper. Praesent sem ligula, tincidunt sed laoreet et, egestas sit amet
 sapien. In eleifend finibus ante, semper congue orci ornare nec. Morbi laoreet mi tortor. Duis
 pretium magna quis arcu consectetur ornare. Nam viverra odio mauris, eu maximus elit maximus quis.
 Aenean sit amet hendrerit leo, at faucibus massa.
 .
 Aenean ac purus pulvinar, efficitur dolor in, posuere lorem. Donec hendrerit enim id sem viverra
 iaculis. Integer quis laoreet lectus, elementum convallis ipsum. Mauris tempus eu tortor eu
 rhoncus. Curabitur non lacus at mi cursus consequat a nec elit. Pellentesque sodales volutpat quam,
 ut ullamcorper nibh egestas in. In leo tortor, dignissim et sagittis vel, dignissim at ex. Aliquam
 maximus odio a nunc finibus imperdiet.
 .
 Aenean eu ex luctus, cursus dolor eget, mattis eros. Fusce nec massa vel nibh gravida tempor. Nunc
 nec lacinia dolor, fermentum varius enim. Cras aliquam mi vel fringilla luctus. Proin aliquet
 faucibus viverra. Fusce blandit turpis ipsum, sed bibendum metus ullamcorper maximus. Aliquam
 facilisis vulputate ligula, ac ultricies elit vestibulum nec. Duis tristique nunc non consequat
 varius. Donec sit amet massa ut urna maximus porttitor. Aliquam vehicula sit amet magna eu finibus.
 Maecenas fringilla dictum elit quis eleifend. Nam porttitor tincidunt venenatis.
"
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
