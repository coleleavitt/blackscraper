PRAGMA foreign_keys = ON;

--
-- schema
--

CREATE TABLE facility (title TEXT, location TEXT, fid INT PRIMARY KEY);
CREATE TABLE item (title TEXT, count INT, iid INT PRIMARY KEY, fid INT,
  FOREIGN KEY (fid) REFERENCES facility(fid));
CREATE TABLE customer (title TEXT, type TEXT, cid INT PRIMARY KEY);

--
-- data
--

-- facility
INSERT INTO facility VALUES ('Flat Rock Assembly', 'Michigan', 1);
INSERT INTO facility VALUES ('Chicago Assembly', 'Illinois', 2);
INSERT INTO facility VALUES ('Dearborn Truck', 'Michigan', 3);
INSERT INTO facility VALUES ('KC Assembly', 'Missouri', 4);

-- item (car)
INSERT INTO item VALUES('f-150', 50, 1, 4);
INSERT INTO item VALUES('fusion', 175, 2, 4);
INSERT INTO item VALUES('mustang', 15, 3, 2);
INSERT INTO item VALUES('explorer', 137, 4, 3);
INSERT INTO item VALUES('flex', 201, 5, 1);

-- customer
INSERT INTO customer VALUES ('USA', 'Govt', 1);
INSERT INTO customer VALUES ('TPD', 'Govt', 2);
INSERT INTO customer VALUES ('AZ Pest Control', 'Org', 3);
INSERT INTO customer VALUES ('Ben Dicken', 'Indiv', 4);
INSERT INTO customer VALUES ('John Doe', 'Indiv', 5);
INSERT INTO customer VALUES ('John Wick', 'Indiv', 6);

--
-- relationship
--

CREATE TABLE purchase (cid, iid, quantity,
    FOREIGN KEY (cid) REFERENCES customer(cid),
    FOREIGN KEY (iid) REFERENCES item(iid));

INSERT INTO purchase VALUES (2, 4, 30);
INSERT INTO purchase VALUES (1, 5, 50);
INSERT INTO purchase VALUES (6, 3, 1);

--
-- queries
--

SELECT * FROM purchase;

SELECT * FROM purchase
  JOIN customer ON purchase.cid == customer.cid;

SELECT * FROM purchase
  JOIN customer ON purchase.cid == customer.cid
  JOIN item ON purchase.iid == item.iid;

SELECT customer.title, item.title, purchase.quantity FROM purchase
  JOIN customer ON purchase.cid == customer.cid
  JOIN item ON purchase.iid == item.iid;

SELECT facility.title, facility.location, item.title FROM item
  JOIN facility ON facility.fid == item.fid;
