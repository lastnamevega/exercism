fn multiples_less_than_limit(limit: u32, factors: &[u32]) -> Vec<u32> {
    let mut multiples: Vec<u32> = vec![];

    for i in 1..limit {
        let mut is_multiple = false;

        for factor in factors {
            if is_multiple || *factor == 0 {
                break;
            }

            if i % factor == 0 {
                is_multiple = true;
            }
        }

        if is_multiple {
            multiples.push(i);
        }
    }

    multiples
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;

    for multiple in multiples_less_than_limit(limit, factors) {
        sum += multiple;
    }

    sum
}
