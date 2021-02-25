 
class Node:
    def __init__(self, pubkey=0, prvkey=0, balance=0, notes=[]):
        self.pubkey = pubkey
        self.prvkey = prvkey
        self.balance = balance
        self.notes = notes

    def make_note(self, value):
        
        # TODO: call the ethStark api to provide a ZK proof
        # proof = secure_prove(str(value))
        proof = hash(str(value))

        newNote = SecretNote(value, proof, self.pubkey)

        self.balance += value
        self.notes.append(newNote)

    def prove_split(self, note, val1, val2):

        # check the note and values
        if note not in self.notes or note.value > self.balance:
            print('Fail to split the note: note not found')
            return False

        if note.value != val1 + val2:
            print('Fail to split the note: sum of the new values is not equal to the old one.')
            return False
        
        # TODO: verify the proof of the old note
        if hash(str(note.value)) != note.proof:
            print('Fail to split the note: invalid note proof')
            return False

        # create new notes
        # TODO: call the ethStark api to provide a ZK proof
        # proof1 = secure_prove(str(val1))
        # proof2 = secure_prove(str(val2))
        proof1 = hash(str(val1))
        proof2 = hash(str(val2))

        newNote1 = SecretNote(val1, proof1, self.pubkey)
        newNote2 = SecretNote(val2, proof2, self.pubkey)

        # self.notes.remove(note)
        self.notes.append(newNote1)
        self.notes.append(newNote2)

        print('Note '+str(note.value)+' is split into new notes '+str(val1)+' and '+str(val2))
        return True

    # TODO: interact with smart contract
    def submit_secret_note_tx(self, smartContract, receiver, note0, note1, note2):
        smartContract.encoded_register_split(note0.proof, note1.proof, note2.proof)
        smartContract.client_submit_tx(self.pubkey, receiver, note2)

    # TODO: merge notes
    def merge_note(self, old1, old2):
        return 1

    

# Registry of notes
class NoteReg:
    def __init__(self, nodes=[], currentNotes=[]):
        self.nodeList = nodes
        self.currentNotes = currentNotes

    def addNode(self, newNode):
        if newNode in self.nodeList:
            print('Node is already in the list')
        else:
            self.nodeList.append(newNode)
        return True
    
    def deleteNode(self, node):
        if node in self.nodeList:
            for note in self.currentNotes:
                if note.owner == node:
                    self.currentNotes.remove(note)
            self.nodeList.remove(node)
        else:
            print('Fail to delete the node: node not found in the list')
            return False

        return True

    def addNote(self, note, owner):
        if owner not in self.nodeList:
            self.addNode(owner)

        if note in self.currentNotes:
            print('Note existed')
            return False
        
        self.currentNotes.append(note)
        return True
        
    def deleteNote(self, note, owner):
        if owner not in self.nodeList:
            print('Fail to delete the note: unregistered node')
            return False

        if note not in self.currentNotes:
            print('Fail to delete the note: note not found')
            return False

        self.currentNotes.remove(note)
        return True

# Secret note class
class SecretNote:
    def __init__(self, value, proof, owner):
        self.value = value
        self.proof = proof
        # owner's public key
        self.owner = owner
