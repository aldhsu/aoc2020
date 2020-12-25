const DIVISOR: u64 = 20201227;
const SUBJECT_NUMBER: u64 = 7;

fn pk_to_loop_size(input: &u64) -> u64 {
    let mut value = 1;
    let mut loop_count = 0;
    while &value != input {
        loop_count += 1;
        value *= SUBJECT_NUMBER;
        value %= DIVISOR;
    }

    loop_count
}

fn pk_to_encryption_key(pk: &u64, loop_size: &u64) -> u64 {
    let mut value = 1;
    let mut loop_count = 0;
    while &loop_count != loop_size {
        loop_count += 1;
        value *= pk;
        value %= DIVISOR;
    }

    value
}

fn main() {
    let card_pk = "15628416".parse::<u64>().unwrap();
    let door_pk = "11161639".parse::<u64>().unwrap();
    let card_loops = pk_to_loop_size(&card_pk);
    let door_loops = pk_to_loop_size(&door_pk);
    let ek = pk_to_encryption_key(&card_pk, &door_loops);
    println!("part1: {}", ek);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_work_out_loop_size() {
        assert_eq!(pk_to_loop_size(&5764801), 8);
        assert_eq!(pk_to_loop_size(&17807724), 11);
    }

    #[test]
    fn it_can_work_out_encryption_key() {
        assert_eq!(pk_to_encryption_key(&17807724, &8), 14897079);
        assert_eq!(pk_to_encryption_key(&5764801, &11), 14897079);
    }
}
