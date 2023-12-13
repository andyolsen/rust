-- Create MYSCHEMA schema.
CREATE SCHEMA MYSCHEMA;

CREATE TABLE MYSCHEMA.EMPLOYEES (
	EmployeeID INT NOT NULL AUTO_INCREMENT,
	Name VARCHAR(50) NOT NULL,
	Salary DOUBLE NOT NULL,
	Region VARCHAR(50) NOT NULL,
	PRIMARY KEY (EmployeeID)
);

CREATE TABLE MYSCHEMA.EMPLOYEES2 (
	EmployeeID INT NOT NULL AUTO_INCREMENT,
	Name VARCHAR(50) NOT NULL,
	Salary DOUBLE NOT NULL,
	Region VARCHAR(50) NOT NULL,
	Photo BLOB(16),
	PRIMARY KEY (EmployeeID)
);

CREATE TABLE MYSCHEMA.SKILLS (
	SkillID INT NOT NULL AUTO_INCREMENT,
	EmployeeID INT NOT NULL,
	Description VARCHAR(50) NOT NULL,

	Level INT DEFAULT 5,
	PRIMARY KEY (SkillID),
	CONSTRAINT EmployeeFK FOREIGN KEY (EmployeeID) REFERENCES MYSCHEMA.EMPLOYEES(EmployeeID)
);

CREATE TABLE MYSCHEMA.CONTRACTS (
	EmployeeID INT NOT NULL,
	StartDate DATE NOT NULL,
	StartSalary DOUBLE NOT NULL,

	PRIMARY KEY (EmployeeID),
	CONSTRAINT EmployeeContractFK FOREIGN KEY (EmployeeID) REFERENCES MYSCHEMA.EMPLOYEES(EmployeeID)
);

CREATE TABLE MYSCHEMA.PERSONS (
	PersonID INT NOT NULL AUTO_INCREMENT,
	FirstName VARCHAR(50) NOT NULL,
	LastName  VARCHAR(50) NOT NULL,
	Address1  VARCHAR(50) NOT NULL,
	Address2  VARCHAR(50) NOT NULL,
	Address3  VARCHAR(50) NOT NULL,
	PRIMARY KEY (PersonID)
);

CREATE TABLE MYSCHEMA.DEPARTMENTS(
	DepartmentID INT NOT NULL AUTO_INCREMENT,
	RegulatoryName VARCHAR(50),
	CommonName VARCHAR(50),
	PRIMARY KEY (DepartmentID)
);


-- Populate tables.
USE MYSCHEMA;

-- Populate EMPLOYEES table.
INSERT INTO MYSCHEMA.EMPLOYEES (Name, Salary, Region) VALUES 
	('Andy',   25000.00, 'South Wales'),
	('Claire', 37000.00, 'Kent'),
	('Mary',   42000.00, 'London'),
	('Mungo',  47000.00, 'Cumbria'),
	('Midge',  72000.00, 'Scotland'),
	('Hayley', 69000.00, 'Northern Ireland'),
	('Nicki',  22000.00, 'Kent'),
	('Sara',   11000.00, 'Kent'),
	('Fiona',  88000.00, 'Kent');

	
-- Populate EMPLOYEES2 table.
INSERT INTO MYSCHEMA.EMPLOYEES2 (Name, Salary, Region) VALUES 
	('Andy',   25000.00, 'South Wales'), 
	('Claire', 37000.00, 'Kent'),
	('Mary',   42000.00, 'London'),
	('Mungo',  47000.00, 'Cumbria'),
	('Midge',  72000.00, 'Scotland'),
	('Hayley', 69000.00, 'Northern Ireland'),
	('Nicki',  22000.00, 'Kent'),
	('Sara',   11000.00, 'Kent'),
	('Fiona',  88000.00, 'Kent');

	
-- Populate SKILLS table.
INSERT INTO MYSCHEMA.SKILLS (EmployeeID, Description, Level) VALUES 
	(1, 'Football', 5), 
	(1, 'Skiing',   3),
	(1, 'Running',  3),
	(2, 'Sales',    4),
	(2, 'Skiing',   4),
	(2, 'Football', 4),
	(3, 'Maths',    5),
	(3, 'Singing',  4),
	(3, 'Teaching', 5),
	(3, 'Running',  2);

	
-- Populate CONTRACTS table.
INSERT INTO MYSCHEMA.CONTRACTS (EmployeeID, StartDate, StartSalary) VALUES 
	(1,  '2008-1-1',  10000), 
	(2,  '2009-1-2',  20000),
	(3,  '2009-1-3',  30000),
	(4,  '2009-1-4',  40000),
	(5,  '2009-1-5',  50000),
	(6,  '2009-1-6',  60000),
	(7,  '2009-1-7',  70000),
	(8,  '2009-1-8',  80000),
	(9,  '2009-1-9',  90000);
	
	
-- Populate PERSONS table.
INSERT INTO MYSCHEMA.PERSONS (FirstName, LastName, Address1, Address2, Address3) VALUES
	('John', 'Smith', '1 Main St', 'Weston', 'Bath'),
	('Jane', 'Evans', '2 High St', 'Newton', 'Neath'),
	('Bill', 'Jones', '3 Oaks St', 'Denton', 'Leeds');

	
-- Populate DEPARTMENTS table.
INSERT INTO MYSCHEMA.DEPARTMENTS(RegulatoryName, CommonName) VALUES
	('MKT', 'Markets'),
	('ACC',  NULL),
	('HR',   NULL),
	(NULL,  'R&D'),
	(NULL,  'Admin');
