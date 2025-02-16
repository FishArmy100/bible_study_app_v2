
export function* range(start: number, end: number): Generator<number>
{
    for(let i = start; i < end; i++)
    {
        yield i;
    }
}

export function* range_inclusive(start: number, end: number): Generator<number>
{
    for(let i = start; i <= end; i++)
    {
        yield i;
    }
}