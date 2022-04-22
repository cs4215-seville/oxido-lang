fn main() {
    fn first<'a>(x: &'a str, y: &'a str) -> &'a str {
        let second = y;
        println("second is {}", second);
        x
    }

    let string1 = StringFrom("first");

    {
        let string2 = StringFrom("second");
        let result = first(AsStr(string1), AsStr(string2));
        println("The first string is {}", result);
    }
}