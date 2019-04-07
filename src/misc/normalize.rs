pub fn normalize<'a, I: 'static>(arr: I) -> Box<dyn Iterator<Item=f32>>
    where I: Iterator<Item=&'a f32> + Clone,
{
    use std::f32;
    let (min, max): (f32, f32) = arr.clone()
                                    .fold((f32::INFINITY, -f32::INFINITY), |(min, max), &v| {
                                        (min.min(v), max.max(v))
                                    });

    debug!("normalize, max {}, min {}", &max, &min);

    Box::new(arr.map(move |&x| (x.clone() - min) / (max - min)))
}

pub fn normalize_vec<'a>(mut arr: Vec<f32>) -> Vec<f32>
{
    use std::f32;
    let (min, max) = arr.iter()
                        .fold((f32::INFINITY, -f32::INFINITY), |(min, max), &v| {
                            (min.min(v), max.max(v))
                        });

    debug!("normalize, max {}, min {}", &max, &min);

    arr.iter_mut().for_each(|x| *x = (*x - min) / (max - min));
    arr
}

pub fn normalize_mut<'a, I>(arr: I)
    where I: Iterator<Item=&'a mut f32> + Clone,
{
    use std::f32;
    let (min, max) = arr.clone()
                        .fold((f32::INFINITY, -f32::INFINITY), |(min, max), &mut v| {
                            (min.min(v), max.max(v))
                        });

    debug!("normalize, max {}, min {}", &max, &min);

    arr.for_each(|x| *x = (*x - min) / (max - min));
}

pub fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

pub fn float_max<'a, I>(arr: I) -> f32
    where I: Iterator<Item=&'a f32>,
{
    use std::f32;
    arr.fold(-f32::INFINITY, |max, &v| {
        max.max(v)
    })
}

pub fn lerp(a: f32, b: f32, x: f32) -> f32 {
    a * x + b * (1. - x)
}

