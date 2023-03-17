BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "person" (
	"matr_nr"	TEXT NOT NULL,
	"firstname"	TEXT,
	"lastname"	TEXT,
	"email"	TEXT,
	"creation_date"	TEXT,
	PRIMARY KEY("matr_nr")
);
CREATE TABLE IF NOT EXISTS "category" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "storage_place" (
	"id"	INTEGER,
	"name"	TEXT,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "item_cat" (
	"id_item"	TEXT,
	"id_category"	INTEGER,
	PRIMARY KEY("id_item","id_category")
);
CREATE TABLE IF NOT EXISTS "lend" (
	"id"	INTEGER NOT NULL,
	"id_lab_item"	INTEGER,
	"id_person"	TEXT,
	"lend_date"	INTEGER,
	"planned_return_date"	TEXT,
	"actual_return_date"	TEXT,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "damage" (
	"id"	INTEGER,
	"lab_item_id"	TEXT,
	"date"	TEXT,
	"description"	TEXT,
	"repaired"	INTEGER,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "lab_item" (
	"id"	TEXT,
	"name"	TEXT,
	"serial_number"	TEXT,
	"price"	INTEGER,
	"buy_date"	TEXT,
	"inventoried"	INTEGER,
	"available"	INTEGER,
	"id_place"	INTEGER
);
CREATE TABLE IF NOT EXISTS "config" (
	"key"	TEXT NOT NULL,
	"value"	TEXT NOT NULL,
	PRIMARY KEY("key")
);
INSERT INTO "person" VALUES ('1234567','Max','Mustermann','max.mustermann@study.thws.de', '2023-01-01');
INSERT INTO "person" VALUES ('ABC1234','Erika','Musterfrau','erika.musterfrau@study.thws.de', '2023-01-01');



INSERT INTO "category" VALUES (1,'Smartphone');
INSERT INTO "category" VALUES (5,'Mikrocontroller');
INSERT INTO "category" VALUES (14,'Linux');
INSERT INTO "category" VALUES (17,'Schwarz');

INSERT INTO "storage_place" VALUES (1,'Labor');
INSERT INTO "storage_place" VALUES (2,'Büro');

INSERT INTO "item_cat" VALUES ('pine_phone_a',1);
INSERT INTO "item_cat" VALUES ('pine_phone_a',14);
INSERT INTO "item_cat" VALUES ('pine_phone_a',17);
INSERT INTO "item_cat" VALUES ('pine_phone_b',1);
INSERT INTO "item_cat" VALUES ('pine_phone_b',14);
INSERT INTO "item_cat" VALUES ('pine_phone_b',17);
INSERT INTO "item_cat" VALUES ('olimexino',5);
INSERT INTO "item_cat" VALUES ('olimexino',17);

INSERT INTO "lend" VALUES (1,'pine_phone_a','1234567','2022-10-18','2022-12-18',NULL);
INSERT INTO "lend" VALUES (2,'pine_phone_a','ABC1234','2022-05-03','2022-07-03','2022-07-11');
INSERT INTO "lend" VALUES (3,'pine_phone_b','1234567','2022-11-25','2022-12-25',NULL);
INSERT INTO "lend" VALUES (4,'pine_phone_b','ABC1234','2022-08-29','2022-09-29','2022-09-18');

INSERT INTO "damage" VALUES (1,'pine_phone_a','2022-12-11','Displayschaden',1);
INSERT INTO "damage" VALUES (2,'pine_phone_a','2022-11-25','Kratzer am Gehäuse',0);
INSERT INTO "damage" VALUES (3,'pine_phone_a','2022-11-26','Akkuschaden',1);
INSERT INTO "damage" VALUES (10,'olimexino','2022-05-19','Pin abgebrochen',1);

INSERT INTO "lab_item" VALUES ('pine_phone_a','PinePhone Pro Explorer Edition','N9TT-9G0A-B7FQ-RANC',42350,'2020-10-04',1,0,1);
INSERT INTO "lab_item" VALUES ('pine_phone_b','PinePhone Pro Explorer Edition','HEY9-EFXB-UH56-D465',42350,'2020-10-04',1,0,1);
INSERT INTO "lab_item" VALUES ('olimexino','OLIMEXINO-STM32','9Q9Q-Z3YF-PVEJ-4AD5',1995,'2022-06-24',1,1,2);

INSERT INTO "config" VALUES ('imprint_text','<p>Dies ist das Impressum</p>');
INSERT INTO "config" VALUES ('imprint_html','1');
INSERT INTO "config" VALUES ('privacy_text','Dies ist die
Datenschutzerklärung<br>');
INSERT INTO "config" VALUES ('privacy_html','0');
INSERT INTO "config" VALUES ('days_until_anonymize','100');
COMMIT;
