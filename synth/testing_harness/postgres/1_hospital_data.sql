-- Hospitals

INSERT INTO public.hospitals (id,hospital_name,address,specialities) VALUES
(1,'Garcia-Washington','194 Davis Ferry Suite 232\nJenningsmouth, NV 83701', '{"Neurology"}'),
(2,'Cruz, Bowman and Martinez','1938 Key Wall\nMartinshire, OR 24041', '{"Cardiology"}'),
(3,'Bishop, Hartman and Zuniga','574 Snyder Crossing\nPort Christineland, VT 37567', '{"Neurology"}'),
(4,'Maxwell-Garcia','328 Williams Coves\nSmithside, HI 71878', NULL),
(5,'Potter-Lindsey','5737 Carmen Trace Suite 312\nSouth Evelyn, WY 40089', NULL),
(6,'Nielsen-Sanchez','70964 Carrillo Burg\nSouth Karichester, ID 67549', '{"Neurology"}'),
(7,'Burch-Daniels','Unit 4839 Box 1083\nDPO AA 25986', '{"Cardiology"}'),
(8,'Marshall, Anderson and Jarvis','51322 Joseph Park\nMelissaton, AZ 67575', '{"Cardiology"}'),
(9,'Nelson-Jones','8068 David Turnpike\nDelgadoside, FL 82542', NULL),
(10,'Hall, Wells and Salas','5280 Kelley Crossroad Apt. 574\nLake Davidfort, CT 94005', NULL),
(11,'Hardy-Obrien','19920 Brian Curve Suite 711\nThompsonville, KY 89805', '{"Neurology", "Cardiology"}'),
(12,'Ayala LLC','0079 Michelle Skyway Suite 179\nPort Tony, CA 48596', '{"Neurology"}'),
(13,'Hale-Padilla','19876 Carroll Flats\nClaytonbury, IA 94229', '{"Neurology", "Cardiology"}'),
(14,'Jones Inc','82451 Anita Rue Suite 317\nJustintown, WI 30269', '{"Cardiology", "Neurology"}');

-- Doctors

INSERT INTO public.doctors (id,hospital_id,name,date_joined) VALUES
(1,1,'Bonnie Johnson','2011-05-25'),
(2,2,'Ian Garrett','2019-03-30'),
(3,1,'Brittney Rowe','2015-01-12'),
(4,3,'Mary Pierce','2019-07-05'),
(5,4,'Cynthia Mendoza','2011-05-16'),
(6,3,'Raymond Bates','2015-01-06'),
(7,5,'Connie Johnson','2019-09-14'),
(8,6,'Maria Rowland','2011-09-04'),
(9,5,'Nicole Taylor','2016-01-27'),
(10,7,'Kenneth Sweeney','2015-11-22'),
(11,8,'Patrick Adams','2017-06-27'),
(12,7,'Shannon Smith','2017-06-13'),
(13,9,'Sydney Walker','2014-07-23'),
(14,10,'Tracy Simon','2015-06-18'),
(15,9,'Mark Harris','2010-11-07'),
(16,11,'Mary Wilson','2012-01-29'),
(17,12,'Danny White','2012-08-12'),
(18,11,'Ashley Manning','2017-01-08'),
(19,13,'Kristine Montgomery','2017-05-22'),
(20,14,'Joshua Ford','2018-07-29'),
(21,13,'Jason Payne','2016-03-03');

-- Patients

INSERT INTO public.patients (id,doctor_id,name,gender,date_joined,address,phone,ssn) VALUES
(1,1,'Thomas Dixon','male','2010-02-24','Studio 6\nCross lock\nEmilyside\nS2E 5RY','(0114)4960562','145-94-5022'),
(2,2,'Anthony Morales','male','2013-07-06','Flat 52D\nAkhtar overpass\nNew Bernardshire\nLN8 1RE','+441154960433','812-01-4209'),
(3,3,'Curtis West','male','2010-05-06','6 Fox roads\nSouth Martynchester\nHS3W 1TT','(0116) 496 0076','548-03-1303'),
(4,1,'Nicholas Ryan','male','2010-10-31','90 Grace radial\nEast Terenceside\nRH87 6GH','(028) 9018 0291','504-87-6136'),
(5,2,'Kristy Rodriguez','unspecified','2017-08-27','035 Oliver forks\nThompsonborough\nB6 2NE','(0808)1570377','689-22-2477'),
(6,3,'Tracy Pacheco','female','2011-12-28','Studio 52\nDorothy tunnel\nMartinstad\nRH69 3AA','(0121)4960038','431-83-7953'),
(7,1,'Tracy Stevens','female','2010-10-17','Flat 74\nAndrew manor\nDianaland\nMK5A 2ZT','01632 960167','552-39-8225'),
(8,2,'Thomas Scott','male','2012-01-04','Flat 26q\nPowell manor\nBarrymouth\nM2 0QU','01184960712','151-56-5875'),
(9,3,'Tammy Charles','unspecified','2013-10-23','798 Jenkins harbor\nNorth Jeremy\nIG49 7YS','+44161 496 0324','474-29-4412'),
(10,1,'Jeanette Nichols','female','2010-04-06','3 Valerie tunnel\nNorth Colin\nSA2N 2QR','+44113 496 0050','378-64-1750');
INSERT INTO public.patients (id,doctor_id,name,gender,date_joined,address,phone,ssn) VALUES
(11,4,'Brett White','male','2010-11-13','Studio 31\nHughes mountains\nEast Norman\nTS23 7JW','+44909 8790083','548-42-7713'),
(12,5,'Raymond Rodriguez','male','2017-06-10','Studio 23a\nSam turnpike\nEmmaview\nOX3P 1DN','+44(0)909 8790659','079-84-1510'),
(13,6,'Lisa Johnson','female','2018-12-12','Flat 7\nRachel hill\nJenniferfurt\nG2 0QD','(0161)4960608','865-55-3656'),
(14,4,'Christopher Gilbert','male','2010-12-29','Studio 0\nJulie motorway\nWatersville\nEN98 4SJ','(0808)1570048','121-28-3435'),
(15,5,'Dr. Evan Garcia','unspecified','2012-02-11','688 Glenn cliff\nAmberton\nN0 4PZ','(0161) 4960368','159-49-5330'),
(16,6,'Christine Phillips','unspecified','2012-07-06','2 Damian lights\nReecemouth\nMK15 1JQ','+44(0)117 4960500','725-75-1763'),
(17,4,'Joseph Stone','male','2011-12-10','203 Rachael union\nJeanmouth\nG07 8DA','+44121 4960898','839-95-0445'),
(18,5,'Donald Ford','male','2013-12-29','064 Glover forges\nNorth Philipport\nL88 8QU','01144960306','197-85-6864'),
(19,6,'Robert Mayer','unspecified','2017-03-30','Studio 91n\nLambert squares\nBrownside\nB6 9YP','01174960906','696-18-2664'),
(20,4,'Deborah Watkins','female','2011-01-19','0 Paul port\nNorth Benjamin\nL7 0ZX','+44117 4960736','862-01-7079');
INSERT INTO public.patients (id,doctor_id,name,gender,date_joined,address,phone,ssn) VALUES
(21,7,'Hannah Kelly','unspecified','2018-01-17','66 Billy turnpike\nNorth Melissa\nAL34 2RG','(0113) 4960268','388-46-1305'),
(22,8,'Jesus Davis','male','2014-11-28','0 Taylor path\nNorth Judithville\nSY6 8SA','+44116 496 0687','020-44-4656'),
(23,9,'Tammy Green','female','2015-08-06','5 Thomas mountain\nSouth Callumland\nE9C 0LQ','+44306 999 0527','086-87-2967'),
(24,7,'Gina Hunt','female','2014-09-15','59 Gordon bridge\nPort Sandraberg\nWS8A 2AS','0151 496 0045','449-82-8506'),
(25,8,'Brittany Matthews','female','2012-09-06','1 Walker station\nEast Alanfurt\nIP7 1QN','0141 496 0206','310-40-7955'),
(26,9,'Jason Miller','male','2011-05-16','Studio 91e\nSheila field\nParkerbury\nLU1 3BP','+44(0)115 496 0265','197-90-4619'),
(27,7,'Victoria Phillips','female','2014-11-13','47 Mason highway\nLake Andreaview\nTS1E 3XQ','+44(0)114 4960023','276-80-3097'),
(28,8,'Robin Fowler','unspecified','2019-04-19','Flat 8\nHeather ridge\nJessicahaven\nGL9R 7SX','+44(0)131 496 0765','544-59-7500'),
(29,9,'Gregory Valencia','male','2016-02-19','090 Dunn shoal\nEast Katyville\nL9J 9GQ','+44(0)118 4960683','477-72-8246'),
(30,7,'Maria Fletcher','female','2017-09-12','Flat 53\nRachael station\nNew Jemma\nE0 1JD','+4428 9018615','591-54-6181');
INSERT INTO public.patients (id,doctor_id,name,gender,date_joined,address,phone,ssn) VALUES
(31,10,'Jennifer Medina','unspecified','2017-07-12','0 Murphy track\nJohnsonmouth\nW8W 8GZ','01164960054','172-54-0780'),
(32,11,'Mr. Brian Porter','unspecified','2016-10-29','Flat 4\nRice camp\nGoodwinview\nW20 5XZ','01164960868','179-79-1760'),
(33,12,'Mark Young','male','2011-03-16','Flat 75\nCarol radial\nNorth Valerie\nHX62 6QA','(0114) 4960210','579-70-1181'),
(34,10,'Michael Nicholson','male','2017-11-16','53 Nicole camp\nGeoffreyside\nNN3H 1FH','(0141) 4960237','155-92-3714'),
(35,11,'George Mcguire','male','2013-11-07','189 Gould overpass\nCookefurt\nHD9 9RF','+44141 4960493','113-40-5786'),
(36,12,'Karen Watkins','female','2019-11-21','76 Carol view\nNew Stephanie\nRH7H 0FQ','+441314960135','567-67-7368'),
(37,10,'Stephanie Gamble','female','2014-08-09','97 Ann flats\nEast Charleneside\nWC7 1HT','+44121 4960213','250-03-1755'),
(38,11,'Kim Meyers','female','2018-11-26','409 Butcher light\nLake Victoriafort\nB65 5FE','(0117)4960932','739-57-9547'),
(39,12,'Ashley Harris','female','2012-05-19','06 Matthews mews\nWoodshire\nUB82 1JY','+44(0)121 4960110','723-90-8396'),
(40,10,'Justin Kim','unspecified','2010-11-20','Flat 25\nLisa inlet\nPort Ritafurt\nHS41 5ER','+44(0)118 496 0663','823-58-6236');
INSERT INTO public.patients (id,doctor_id,name,gender,date_joined,address,phone,ssn) VALUES
(41,13,'Mrs. Kristin Lewis DVM','female','2010-12-25','00 Barnes wells\nNew Sian\nS00 8YR','(020) 74960924','156-29-6887'),
(42,14,'Janet Fernandez','unspecified','2016-11-29','Flat 79\nNorman run\nSharonmouth\nB4U 7BL','+44(0)115 496 0126','460-18-0819'),
(43,15,'Leslie Casey','female','2018-05-19','66 Smith expressway\nNormantown\nL1 5WR','+44(0)306 999 0764','067-93-6633'),
(44,13,'Samantha Wright','female','2015-02-03','629 Burton gateway\nHughville\nS9 3LJ','0118 4960244','738-95-0807'),
(45,14,'Brett Miller','male','2018-04-11','436 Graham hill\nShirleyborough\nPL5Y 3LD','0141 496 0793','726-38-6007'),
(46,15,'Jason Crane','unspecified','2017-04-26','Flat 99\nYoung fall\nAliceton\nB0S 2AT','+44(0)1174960479','833-04-0115'),
(47,13,'Madison Yoder','female','2015-03-23','062 Katy landing\nPhillipsview\nSS91 9YP','01184960060','006-10-0679'),
(48,14,'Michael Sellers','male','2019-10-28','142 Stephen junction\nBennettfurt\nW2W 9BD','+44(0)8081570284','472-17-6148'),
(49,15,'Rebecca Brown','unspecified','2011-10-20','Flat 64\nDennis throughway\nEast Benjaminland\nDT37 3FL','(0117) 4960069','233-17-3594'),
(50,13,'Cody Bryant','male','2015-08-23','Studio 5\nLewis hollow\nEdwardsland\nS96 4TG','+44115 496 0334','649-23-3293');
INSERT INTO public.patients (id,doctor_id,name,gender,date_joined,address,phone,ssn) VALUES
(51,16,'Kevin Payne','male','2019-03-24','Studio 8\nBethan stream\nNew Elliottshire\nSL7H 0SS','+441314960539','585-82-2591'),
(52,17,'Megan Ryan','female','2016-04-21','Flat 6\nHenry inlet\nWest Samantha\nHS7N 9FT','+44(0)116 4960007','698-18-1758'),
(53,18,'Matthew Henry','unspecified','2018-10-20','743 Shaw knoll\nLeeside\nM01 3HY','01154960297','775-07-6124'),
(54,16,'Noah Chandler','male','2013-03-31','Studio 73\nBell loaf\nAlitown\nCA0 8AY','+44121 4960245','761-56-1725'),
(55,17,'Garrett Fowler','male','2011-07-17','Flat 23A\nMitchell squares\nHoltchester\nE23 9WJ','+441632 960 338','859-35-9693'),
(56,18,'Spencer Campbell','male','2013-06-16','915 Mason bridge\nSouth Patriciachester\nDA61 0ZN','+44(0)808 157 0553','363-69-2489'),
(57,16,'Robert Becker','male','2018-11-27','900 Boyle forest\nPort Andrewburgh\nM6 2HT','(020) 7496 0080','788-71-1402'),
(58,17,'Matthew Murillo','male','2015-03-01','Studio 30\nHughes river\nWhitemouth\nS10 0LN','02074960325','330-99-9498'),
(59,18,'Brandy Hernandez','male','2018-05-27','Flat 3\nAntony forges\nLake Eric\nPO0 8UP','09098790239','803-29-7495'),
(60,16,'Jamie Sutton','male','2010-11-19','927 Gibbs cliff\nGregoryshire\nUB6H 9XQ','+44(0)29 2018479','645-23-9803');
INSERT INTO public.patients (id,doctor_id,name,gender,date_joined,address,phone,ssn) VALUES
(61,19,'Renee Robertson','female','2019-07-24','0 Jodie pike\nEast Alan\nBH7Y 2WN','(0115) 4960107','787-06-1801'),
(62,20,'Rachel Nelson','unspecified','2012-01-11','Studio 6\nAdam lodge\nNew Maureen\nG4W 3UN','0131 4960314','829-19-7394'),
(63,21,'Melissa Silva','female','2019-05-09','Studio 58\nGreen shore\nNorth Shauntown\nS54 5NY','(0121) 4960580','321-13-7363'),
(64,19,'Frank Bradley','male','2012-09-08','9 Sandra avenue\nJaniceville\nM1 0JP','(0161) 496 0548','187-91-0569'),
(65,20,'Stephen Briggs','unspecified','2010-03-29','Flat 82\nQuinn parkways\nEast Joyceland\nG2 2TX','029 2018 0186','231-59-8966'),
(66,21,'Emily Camacho','female','2016-12-26','093 Marshall plains\nEast Mathewfort\nKY3 5WU','+44(0)909 8790988','534-62-0149'),
(67,19,'Ryan Cain','male','2017-11-18','8 Dawn glens\nLouiseburgh\nE1W 0LH','(0141)4960552','518-81-2366'),
(68,20,'Daniel Thornton','male','2017-01-16','01 Lisa cliff\nSmithmouth\nOX92 5ZQ','01414960391','511-66-0104'),
(69,21,'Kyle Pacheco','male','2018-06-26','754 Thompson lodge\nThomashaven\nDH41 3JW','(028) 9018567','008-70-2086'),
(70,19,'Scott Nielsen','male','2016-10-15','Studio 74y\nBeth path\nMooreburgh\nE2 9ZB','(0808)1570400','823-56-1410');
