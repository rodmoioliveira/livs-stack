CREATE TABLE books (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    isbn VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    editor VARCHAR(255) NOT NULL,
    description TEXT NOT NULL
);

INSERT INTO books(isbn, author, title, editor, description)
VALUES
    ('B07M82PNSX', 'Bill Bryson', 'The Body: A Guide for Occupants', 'Anchor', 'Bill Bryson once again proves himself to be an incomparable companion as he guides us through the human body—how it functions, its remarkable ability to heal itself, and (unfortunately) the ways it can fail. Full of extraordinary facts (your body made a million red blood cells since you started reading this) and irresistible Brysonesque anecdotes, The Body will lead you to a deeper understanding of the miracle that is life in general and you in particular.'),
    ('B07C6YRCGJ', 'Ryan Jacobs ', 'The Truffle Underground: A Tale of Mystery, Mayhem, and Manipulation in the Shadowy Market of the Worlds Most Expensive Fungus', 'Clarkson Potter', 'Beneath the gloss of star chefs and crystal-laden tables, the truffle supply chain is touched by theft, secrecy, sabotage, and fraud. Farmers patrol their fields with rifles and fear losing trade secrets to spies. Hunters plant poisoned meatballs to eliminate rival truffle-hunting dogs. Naive buyers and even knowledgeable experts are duped by liars and counterfeits.');
