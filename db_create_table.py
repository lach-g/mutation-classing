import sqlalchemy as db
import numpy as np
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

# Connect with database
db_engine = db.create_engine('mysql+pymysql://root:password@localhost:3306/testdb')

# Manage tables
db_base = declarative_base()

class Genetics(db_base):
    __tablename__ = 'genetics'
    id = db.Column(db.Integer, primary_key=True)
    gene_1 = db.Column(db.Integer)
    gene_2 = db.Column(db.Integer)
    gene_3 = db.Column(db.Integer)
    gene_4 = db.Column(db.Integer)
    gene_5 = db.Column(db.Integer)
    gene_6 = db.Column(db.Integer)
    gene_7 = db.Column(db.Integer)
    gene_8 = db.Column(db.Integer)
    gene_9 = db.Column(db.Integer)
    gene_10 = db.Column(db.Integer)

    def __init__(self,
                 gene_1,
                 gene_2,
                 gene_3,
                 gene_4,
                 gene_5,
                 gene_6,
                 gene_7,
                 gene_8,
                 gene_9,
                 gene_10,
                 ):
        self.gene_1 = gene_1
        self.gene_2 = gene_2
        self.gene_3 = gene_3
        self.gene_4 = gene_4
        self.gene_5 = gene_5
        self.gene_6 = gene_6
        self.gene_7 = gene_7
        self.gene_8 = gene_8
        self.gene_9 = gene_9
        self.gene_10 = gene_10

db_base.metadata.create_all(db_engine)

# New session
Session = sessionmaker(db_engine)
session = Session()

# Add random data
num_rows = 20
num_cols = 10
for i in range(num_rows):
    rand_genes = np.random.randint(2, size=num_cols)
    row = Genetics(rand_genes[0],
                   rand_genes[1],
                   rand_genes[2],
                   rand_genes[3],
                   rand_genes[4],
                   rand_genes[5],
                   rand_genes[6],
                   rand_genes[7],
                   rand_genes[8],
                   rand_genes[9],
                   )
    session.add(row)

# Save changes
session.commit()
