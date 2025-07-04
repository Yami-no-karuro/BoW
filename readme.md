# Rust - Bag of Words

## Simple BoW implementation, in Rust

### Intro

A **Bag of Words** (BoW) is a basic yet powerful model used in **Natural Language Processing** (NLP) and text mining.  
Its goal is to represent text as a multiset of its words, discarding grammar and word order but preserving multiplicity (i.e., how often each word appears).  
The BoW model transforms raw text into a structured numerical representation, typically a sparse vector, where each dimension corresponds to a specific word from the corpus vocabulary.  
This representation is particularly useful for tasks like **text classification**, **spam detection**, and **information retrieval**.  
(More on [Bag of Words](https://en.wikipedia.org/wiki/Bag-of-words_model) on [Wikipedia](https://en.wikipedia.org))
