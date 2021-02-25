
# Secret note 
class SecretNote:
    def __init__(self, value=0, proof='', owner=''):
        self.value = value
        self.proof = proof
        self.owner = owner

    def __eq__(self, noteB):
        return self.value == noteB.value and self.proof == noteB.proof and self.owner == noteB.owner
    
    def __ne__(self, noteB):
        return not self == noteB

    def __str__(self):
        return 'value: '+str(self.value)+', proof: '+str(self.proof)+', owner: '+self.owner

    def __repr__(self):
        return str(self)


# TODO: external call to ethStark create proof function
def rescue_prove(value):
    return hash(str(value))

# TODO: extrnal call to ethStark verify function
def rescue_verify(proof0, proof1, proof2):
    return True
