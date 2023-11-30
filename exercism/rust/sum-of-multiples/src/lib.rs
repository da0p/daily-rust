use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .copied()
        .filter(|factor| *factor != 0)
        .map(|factor| {
            (1..limit / factor + 1)
                .filter(|r| r * factor < limit)
                .map(|r| factor * r)
                .collect::<Vec<_>>()
        })
        .flat_map(|e| e.into_iter())
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}
