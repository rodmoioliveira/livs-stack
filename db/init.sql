CREATE TABLE IF NOT EXISTS titles (
  id BIGSERIAL NOT NULL,
  isbn BIGINT NOT NULL,
  author VARCHAR(255) NOT NULL,
  title VARCHAR(255) NOT NULL,
  publisher VARCHAR(255) NOT NULL,
  year SMALLINT NOT NULL
);
INSERT INTO
  titles(isbn, author, title, publisher, year)
VALUES
  (
    9788535932317,
    'José Luís Peixoto',
    'Autobiografia',
    'Companhia das Letras',
    2019
  ),
  (
    9788575596296,
    'Ricardo Antunes',
    'O Privilégio da Servidão: O Novo Proletariado de Serviços na era Digital',
    'Boitempo',
    2018
  ),
  (
    9788535932065,
    'Roberto Bolaño',
    'A Literatura Nazista na América',
    'Companhia das Letras',
    2019
  ),
  (
    9788466330961,
    'Juan Carlos Onetti',
    'El Pozo',
    'Debolsillo',
    2018
  ),
  (
    9788576655299,
    'Mary del Priore e Renato Venancio',
    'Uma Breve História do Brasil',
    'Planeta',
    2010
  ),
  (
    9783037781050,
    'Kenya Hara',
    'Designing Design',
    'Lars Müller Publishers',
    2011
  ),
  (
    9788569536499,
    'Sabrina Fernandes',
    'Sintomas Mórbidos',
    'Autonomia Literária',
    2019
  ),
  (
    9780822331971,
    'Ativa Chomsky, Barry Carr e Pamela Maria Smorkaloff',
    'The Cuba Reader: History, Culture, Politics',
    'Duke',
    2003
  ),
  (
    9788535919837,
    'José Saramago',
    'Claraboia',
    'Companhia das Letras',
    2011
  ),
  (
    9788535921700,
    'Mário Magalhães',
    'Marighella: o Guerrilheiro que Incendiou o Mundo',
    'Companhia das Letras',
    2012
  ),
  (
    9781501174476,
    'Stephen Markley',
    'Ohio',
    'Simon & Schuster',
    2018
  );
