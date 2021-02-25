
import common
from common import SecretNote

class Node:
    def __init__(self, pubkey='', prvkey='', balance=0, notes=[]):
        self.pubkey = pubkey
        self.prvkey = prvkey
        self.balance = balance
        self.notes = notes

    def make_note(self, smartContract, value):
        
        proof = common.rescue_prove(value)

        newNote = SecretNote(value, proof, self.pubkey)

        self.balance += value
        self.notes.append(newNote)

        if smartContract.register_note(self.pubkey, newNote):
            print('New note created: ' + str(newNote))
            return True
        return False

    def prove_split(self, note, val1, val2):

        # check the note and values
        if note not in self.notes or note.value > self.balance:
            print('Fail to split the note: note not found')
            return False

        if note.value != val1 + val2:
            print('Fail to split the note: sum of the new values is not equal to the old one.')
            return False
        
        if common.rescue_prove(note.value) != note.proof:
            print('Fail to split the note: invalid note proof')
            return False

        # create new notes
        proof1 = common.rescue_prove(val1)
        proof2 = common.rescue_prove(val2)

        newNote1 = SecretNote(val1, proof1, self.pubkey)
        newNote2 = SecretNote(val2, proof2, self.pubkey)

        # self.notes.remove(note)
        self.notes.append(newNote1)
        self.notes.append(newNote2)

        print('Note '+str(note.value)+' is split into new notes '+str(val1)+' and '+str(val2))
        return newNote1, newNote2

    def submit_secret_note_tx(self, smartContract, receiver, note, value):
        if note.value < value:
            print('Fail to transfer: inadequate balance')
            return False

        txNote = note
        if note.value > value:
            restValue = note.value - value
            restNote, txNote = self.prove_split(note, restValue, value)
            if not smartContract.encoded_register_split(self.pubkey, note.proof, restNote.proof, txNote.proof):
                print('Fail to transfer: cannot split the note')
                return False
            self.notes.remove(note)
        
        if smartContract.client_submit_tx(self.pubkey, receiver, txNote):
            self.balance -= txNote.value
            self.notes.remove(txNote)
            print('Note ('+str(txNote)+') has been sent')
            return True
        
        return False

    # TODO: merge notes
    def merge_note(self, old1, old2):
        return 1
