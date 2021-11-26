-- DELETE STORAGE GROUP
DELETE STORAGE GROUP root.test;

-- CREATE TIMESERIES
CREATE TIMESERIES root.test.status WITH DATATYPE=BOOLEAN, ENCODING=PLAIN;
CREATE TIMESERIES root.test.temperature WITH DATATYPE=FLOAT, ENCODING=RLE;

-- INSERT INTO DATA
INSERT INTO root.test(timestamp, status)
values (1637960249484, true);

-- INSERT INTO DATA
INSERT INTO root.test(timestamp, status, temperature)
values (1637960256493, false, 20.71);

-- INSERT INTO DATA
INSERT INTO root.test(timestamp, status, temperature)
values (1637960261494, true, 32.43);

-- INSERT INTO DATA
INSERT INTO root.test(timestamp, status, temperature)
values (1637960272492, false, 28.66);

-- INSERT INTO DATA
INSERT INTO root.test(timestamp, status, temperature)
values (1637960272492, true, 22.61);

-- INSERT INTO DATA
INSERT INTO root.test(timestamp, status, temperature)
values (1637960296493, false, 28.66);