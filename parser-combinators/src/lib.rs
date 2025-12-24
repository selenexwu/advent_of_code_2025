use std::marker::PhantomData;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorType {
    Msg(String),
    Read,
    Satisfy,
    Choice,
}

#[derive(Debug)]
pub struct ParseError<'a, T> {
    pub typ: ErrorType,
    pub rest: &'a [T],
}

impl<'a, T> ParseError<'a, T> {
    pub fn combine(self, other: ParseError<'a, T>) -> ParseError<'a, T> {
        // TODO: better error combining
        if self.rest.len() < other.rest.len() {
            self
        } else {
            other
        }
    }
}

pub type ParseResult<'a, T, O> = Result<(O, &'a [T]), ParseError<'a, T>>;

pub trait Parser<'a, T: 'a>: Sized {
    type O;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O>;

    fn disj<P>(self, other: P) -> Disj<Self, P>
    where
        P: Parser<'a, T, O = Self::O>,
    {
        Disj {
            p1: self,
            p2: other,
        }
    }

    fn backtrack(self) -> Backtrack<Self> {
        Backtrack(self)
    }

    fn satisfy<F>(self, f: F) -> Satisfy<Self, F>
    where
        F: Fn(&Self::O) -> bool,
    {
        Satisfy { p: self, f: f }
    }

    fn map<F, O2>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::O) -> O2,
    {
        Map { p: self, f: f }
    }

    fn map_const<O2: Clone>(self, x: O2) -> MapConst<Self, O2> {
        MapConst { p: self, x: x }
    }

    fn seql<O2, P2>(self, p2: P2) -> Seql<Self, P2>
    where
        P2: Parser<'a, T, O = O2>,
    {
        Seql { p1: self, p2: p2 }
    }

    fn seqr<O2, P2>(self, p2: P2) -> Seqr<Self, P2>
    where
        P2: Parser<'a, T, O = O2>,
    {
        Seqr { p1: self, p2: p2 }
    }

    fn optional(self) -> Optional<Self> {
        Optional(self)
    }

    fn many(self) -> Many<Self> {
        Many(self)
    }

    fn many1(self) -> Many1<Self> {
        Many1(self)
    }

    fn sep_by<S, PS>(self, sep: PS) -> SepBy<Self, PS>
    where
        PS: Parser<'a, T, O = S>,
    {
        SepBy { p: self, sep: sep }
    }

    fn sep_by1<S, PS>(self, sep: PS) -> SepBy1<Self, PS>
    where
        PS: Parser<'a, T, O = S>,
    {
        SepBy1 { p: self, sep: sep }
    }

    fn end_by<S, PS>(self, sep: PS) -> EndBy<Self, PS>
    where
        PS: Parser<'a, T, O = S>,
    {
        EndBy { p: self, sep: sep }
    }

    fn end_by1<S, PS>(self, sep: PS) -> EndBy1<Self, PS>
    where
        PS: Parser<'a, T, O = S>,
    {
        EndBy1 { p: self, sep: sep }
    }

    fn sep_end_by<S, PS>(self, sep: PS) -> SepEndBy<Self, PS>
    where
        PS: Parser<'a, T, O = S>,
    {
        SepEndBy { p: self, sep: sep }
    }

    fn sep_end_by1<S, PS>(self, sep: PS) -> SepEndBy1<Self, PS>
    where
        PS: Parser<'a, T, O = S>,
    {
        SepEndBy1 { p: self, sep: sep }
    }
}

pub struct Disj<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<'a, T: 'a, O, P1, P2> Parser<'a, T> for Disj<P1, P2>
where
    P1: Parser<'a, T, O = O>,
    P2: Parser<'a, T, O = O>,
{
    type O = O;
    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, O> {
        self.p1.parse(tokens).or_else(|e1| {
            // Check to prevent backtracking by more than 1
            if tokens.len() - e1.rest.len() > 1 {
                Err(e1)
            } else {
                self.p2.parse(tokens).or_else(|e2| Err(e1.combine(e2)))
            }
        })
    }
}

pub struct Backtrack<P>(P);

impl<'a, T: 'a, O, P> Parser<'a, T> for Backtrack<P>
where
    P: Parser<'a, T, O = O>,
{
    type O = O;
    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, O> {
        self.0.parse(tokens).or_else(|ParseError { typ, rest: _ }| {
            Err(ParseError {
                typ: typ,
                rest: tokens,
            })
        })
    }
}

pub struct Satisfy<P, F> {
    p: P,
    f: F,
}

impl<'a, T: 'a, O, P, F> Parser<'a, T> for Satisfy<P, F>
where
    P: Parser<'a, T, O = O>,
    F: Fn(&O) -> bool,
{
    type O = O;
    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, O> {
        let (res, rest) = self.p.parse(tokens)?;
        if (self.f)(&res) {
            Ok((res, rest))
        } else {
            Err(ParseError {
                typ: ErrorType::Satisfy,
                rest: rest,
            })
        }
    }
}

pub struct Map<P, F> {
    p: P,
    f: F,
}

impl<'a, T: 'a, O, O2, P, F> Parser<'a, T> for Map<P, F>
where
    P: Parser<'a, T, O = O>,
    F: Fn(O) -> O2,
{
    type O = O2;
    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, O2> {
        let (res, rest) = self.p.parse(tokens)?;
        Ok(((self.f)(res), rest))
    }
}

pub struct MapConst<P, O> {
    p: P,
    x: O,
}

impl<'a, T: 'a, O, O2: Clone, P> Parser<'a, T> for MapConst<P, O2>
where
    P: Parser<'a, T, O = O>,
{
    type O = O2;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        let (_, rest) = self.p.parse(tokens)?;
        Ok((self.x.clone(), rest))
    }
}

impl<'a, T: 'a, O1, O2, P1, P2> Parser<'a, T> for (P1, P2)
where
    P1: Parser<'a, T, O = O1>,
    P2: Parser<'a, T, O = O2>,
{
    type O = (O1, O2);

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        let (p1, p2) = self;
        let (res1, rest1) = p1.parse(tokens)?;
        let (res2, rest2) = p2.parse(rest1)?;
        Ok(((res1, res2), rest2))
    }
}

pub struct Seql<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<'a, T: 'a, O1, O2, P1, P2> Parser<'a, T> for Seql<P1, P2>
where
    P1: Parser<'a, T, O = O1>,
    P2: Parser<'a, T, O = O2>,
{
    type O = O1;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        let (res, rest1) = self.p1.parse(tokens)?;
        let (_, rest2) = self.p2.parse(rest1)?;
        Ok((res, rest2))
    }
}

pub struct Seqr<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<'a, T: 'a, O1, O2, P1, P2> Parser<'a, T> for Seqr<P1, P2>
where
    P1: Parser<'a, T, O = O1>,
    P2: Parser<'a, T, O = O2>,
{
    type O = O2;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        let (_, rest1) = self.p1.parse(tokens)?;
        let (res, rest2) = self.p2.parse(rest1)?;
        Ok((res, rest2))
    }
}

pub struct Optional<P>(P);

impl<'a, T: 'a, O, P> Parser<'a, T> for Optional<P>
where
    P: Parser<'a, T, O = O>,
{
    type O = Option<O>;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        match self.0.parse(tokens) {
            Ok((res, rest)) => Ok((Some(res), rest)),
            Err(_) => Ok((None, tokens)),
        }
    }
}

pub struct Many<P>(P);

impl<'a, T: 'a, O, P> Parser<'a, T> for Many<P>
where
    P: Parser<'a, T, O = O>,
{
    type O = Vec<O>;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        let mut rest = tokens;
        let mut res = Vec::new();
        while let Ok((one, rest1)) = self.0.parse(rest) {
            rest = rest1;
            res.push(one)
        }
        Ok((res, rest))
    }
}

pub struct Many1<P>(P);

impl<'a, T: 'a, O, P> Parser<'a, T> for Many1<P>
where
    P: Parser<'a, T, O = O>,
{
    type O = Vec<O>;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        let (first, mut rest) = self.0.parse(tokens)?;
        let mut res = vec![first];
        while let Ok((one, rest1)) = self.0.parse(rest) {
            rest = rest1;
            res.push(one)
        }
        Ok((res, rest))
    }
}

pub struct SepBy<P, PS> {
    p: P,
    sep: PS,
}

impl<'a, T: 'a, O, P, PS, S> Parser<'a, T> for SepBy<P, PS>
where
    P: Parser<'a, T, O = O>,
    PS: Parser<'a, T, O = S>,
{
    type O = Vec<O>;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        match self.p.parse(tokens) {
            Err(_) => Ok((Vec::new(), tokens)),
            Ok((first, mut rest)) => {
                let mut res = vec![first];
                while let Ok((_, rest1)) = self.sep.parse(rest)
                    && let Ok((one, rest2)) = self.p.parse(rest1)
                {
                    rest = rest2;
                    res.push(one)
                }
                Ok((res, rest))
            }
        }
    }
}

pub struct SepBy1<P, PS> {
    p: P,
    sep: PS,
}

impl<'a, T: 'a, O, P, PS, S> Parser<'a, T> for SepBy1<P, PS>
where
    P: Parser<'a, T, O = O>,
    PS: Parser<'a, T, O = S>,
{
    type O = Vec<O>;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        let (first, mut rest) = self.p.parse(tokens)?;
        let mut res = vec![first];
        while let Ok((_, rest1)) = self.sep.parse(rest)
            && let Ok((one, rest2)) = self.p.parse(rest1)
        {
            rest = rest2;
            res.push(one)
        }
        Ok((res, rest))
    }
}

pub struct EndBy<P, PS> {
    p: P,
    sep: PS,
}

impl<'a, T: 'a, O, P, PS, S> Parser<'a, T> for EndBy<P, PS>
where
    P: Parser<'a, T, O = O>,
    PS: Parser<'a, T, O = S>,
{
    type O = Vec<O>;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        let mut rest = tokens;
        let mut res = Vec::new();
        while let Ok((one, rest1)) = self.p.parse(rest)
            && let Ok((_, rest2)) = self.sep.parse(rest1)
        {
            rest = rest2;
            res.push(one)
        }
        Ok((res, rest))
    }
}

pub struct EndBy1<P, PS> {
    p: P,
    sep: PS,
}

impl<'a, T: 'a, O, P, PS, S> Parser<'a, T> for EndBy1<P, PS>
where
    P: Parser<'a, T, O = O>,
    PS: Parser<'a, T, O = S>,
{
    type O = Vec<O>;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        let (first, rest1) = self.p.parse(tokens)?;
        let (_, mut rest) = self.sep.parse(rest1)?;
        let mut res = vec![first];
        while let Ok((one, rest1)) = self.p.parse(rest)
            && let Ok((_, rest2)) = self.sep.parse(rest1)
        {
            rest = rest2;
            res.push(one)
        }
        Ok((res, rest))
    }
}

pub struct SepEndBy<P, PS> {
    p: P,
    sep: PS,
}

impl<'a, T: 'a, O, P, PS, S> Parser<'a, T> for SepEndBy<P, PS>
where
    P: Parser<'a, T, O = O>,
    PS: Parser<'a, T, O = S>,
{
    type O = Vec<O>;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        match self.p.parse(tokens) {
            Err(_) => Ok((Vec::new(), tokens)),
            Ok((first, mut rest)) => {
                let mut res = vec![first];
                while let Ok((_, rest1)) = self.sep.parse(rest)
                    && let Ok((one, rest2)) = self.p.parse(rest1)
                {
                    rest = rest2;
                    res.push(one)
                }
                let rest = match self.sep.parse(rest) {
                    Ok((_, rest)) => rest,
                    Err(ParseError { typ: _, rest }) => rest,
                };
                Ok((res, rest))
            }
        }
    }
}

pub struct SepEndBy1<P, PS> {
    p: P,
    sep: PS,
}

impl<'a, T: 'a, O, P, PS, S> Parser<'a, T> for SepEndBy1<P, PS>
where
    P: Parser<'a, T, O = O>,
    PS: Parser<'a, T, O = S>,
{
    type O = Vec<O>;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        let (first, mut rest) = self.p.parse(tokens)?;
        let mut res = vec![first];
        while let Ok((_, rest1)) = self.sep.parse(rest)
            && let Ok((one, rest2)) = self.p.parse(rest1)
        {
            rest = rest2;
            res.push(one)
        }
        let rest = match self.sep.parse(rest) {
            Ok((_, rest)) => rest,
            Err(ParseError { typ: _, rest }) => rest,
        };
        Ok((res, rest))
    }
}

impl<'a, F, T: 'a, O> Parser<'a, T> for F
where
    F: Fn(&'a [T]) -> ParseResult<'a, T, O>,
{
    type O = O;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, O> {
        self(tokens)
    }
}

pub struct PureWith<F>(F);

impl<'a, T: 'a, O, F> Parser<'a, T> for PureWith<F>
where
    F: Fn() -> O,
{
    type O = O;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        Ok(((self.0)(), tokens))
    }
}

pub fn pure_with<F, O>(f: F) -> PureWith<F>
where
    F: Fn() -> O,
{
    PureWith(f)
}

pub struct Pure<O>(O);

impl<'a, T: 'a, O: Clone> Parser<'a, T> for Pure<O> {
    type O = O;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        Ok((self.0.clone(), tokens))
    }
}

pub fn pure<O>(x: O) -> Pure<O> {
    Pure(x)
}

pub struct Fail<O>(ErrorType, PhantomData<O>);

impl<'a, T: 'a, O> Parser<'a, T> for Fail<O> {
    type O = O;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        Err(ParseError {
            typ: self.0.clone(),
            rest: tokens,
        })
    }
}

pub fn fail<O>(typ: ErrorType) -> Fail<O> {
    Fail(typ, PhantomData)
}

pub struct FailWithMessage<O>(String, PhantomData<O>);

impl<'a, T: 'a, O> Parser<'a, T> for FailWithMessage<O> {
    type O = O;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        Err(ParseError {
            typ: ErrorType::Msg(self.0.clone()),
            rest: tokens,
        })
    }
}

pub fn fail_with_message<O>(msg: String) -> FailWithMessage<O> {
    FailWithMessage(msg, PhantomData)
}

pub struct Read();

impl<'a, T: 'a> Parser<'a, T> for Read {
    type O = &'a T;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        match tokens {
            [tok, rest @ ..] => Ok((tok, rest)),
            [] => Err(ParseError {
                typ: ErrorType::Read,
                rest: tokens,
            }),
        }
    }
}

pub fn read() -> Read {
    Read()
}

pub struct Choice<'a, P>(&'a [P]);

impl<'a, T: 'a, P, O> Parser<'a, T> for Choice<'_, P>
where
    P: Parser<'a, T, O = O>,
{
    type O = O;

    fn parse(&self, tokens: &'a [T]) -> ParseResult<'a, T, Self::O> {
        for p in self.0.iter() {
            if let Ok((res, rest)) = p.parse(tokens) {
                return Ok((res, rest));
            }
        }
        Err(ParseError {
            typ: ErrorType::Choice,
            rest: tokens,
        })
    }
}

pub fn choice<'a, T: 'a, P, O>(ps: &'a [P]) -> Choice<'a, P>
where
    P: Parser<'a, T, O = O>,
{
    Choice(ps)
}
