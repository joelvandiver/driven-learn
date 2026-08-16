-- setup.sql — Postgres Module 1: Querying basics
-- Run this first to create and seed the table, then run easy.sql / medium.sql /
-- hard.sql against the same database:
--     psql -f setup.sql
--     psql -f easy.sql
-- (or paste everything into one psql session).

DROP TABLE IF EXISTS books;

CREATE TABLE books (
    id             int PRIMARY KEY,       -- unique row id
    title          text NOT NULL,         -- book title
    author         text NOT NULL,         -- last name is enough for us
    genre          text NOT NULL,         -- 'fiction' or 'tech'
    published_year int  NOT NULL,         -- year of publication
    pages          int  NOT NULL,         -- length in pages
    price          numeric(5,2) NOT NULL  -- price with 2 decimal places
);

INSERT INTO books (id, title, author, genre, published_year, pages, price) VALUES
    (1, 'The Rust Programming Language',           'Klabnik',   'tech',    2018, 560, 39.95),
    (2, 'Dune',                                    'Herbert',   'fiction', 1965, 412,  9.99),
    (3, 'Project Hail Mary',                       'Weir',      'fiction', 2021, 496, 14.99),
    (4, 'The Pragmatic Programmer',                'Hunt',      'tech',    1999, 352, 49.99),
    (5, 'Klara and the Sun',                       'Ishiguro',  'fiction', 2021, 320, 12.50),
    (6, 'Neuromancer',                             'Gibson',    'fiction', 1984, 271,  8.99),
    (7, 'Designing Data-Intensive Applications',   'Kleppmann', 'tech',    2017, 616, 44.99),
    (8, 'The Midnight Library',                     'Haig',      'fiction', 2020, 304, 11.00);

-- Worked example: the 3 cheapest fiction books published after 2000.
SELECT title,
       published_year,
       price
FROM books
WHERE genre = 'fiction'
  AND published_year > 2000
ORDER BY price ASC
LIMIT 3;
