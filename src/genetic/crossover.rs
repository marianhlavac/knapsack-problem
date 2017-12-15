fn simple_half(p1: BitVec, p2: BitVec) -> BitVec {
    let length = p1.len();
    p1.truncate(length / 2);
    p1.append(p2.split_off(length / 2)).truncate(length)
}