/// What should the type of _function be?
pub fn map<T, S, F>(input: Vec<T>, mut function: F) -> Vec<S>
where
    F: FnMut(T) -> S,
{
    let mut output = Vec::new();

    for i in input {
        output.push(function(i));
    }

    output
}
