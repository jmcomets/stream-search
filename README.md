stream-search
=============

An actual coding interview question I was once asked (and flunked).

## Problem statement

Given a huge stream of words in a corpus, I want to find a *best match* for a
query from the user.

This match is defined as the full set of words in the query (in order),
possibly separated by an arbitrary set of other words. The *best* is defined as
the match who minimizes the sum, for each word in the query, of actual word
positions in the corpus.

The interviewer requested an `O(n*k)` time complexity and `O(k)` space
solution, where `n` is the number of words in the stream and `k` is the number
of words in the query.

The current solution is `O(n)` space, since there can be partial matches who
are completed only at the end of the stream.

I have found a `O(k)` space solution and am working on implementing it.
