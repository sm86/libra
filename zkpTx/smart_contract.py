import common
from common import SecretNote

# Registry of notes
# TODO: remove the value of the notes in the registry on-chain?
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

# pseudo-chain
class SmartContract:
    def __init__(self, noteReg=NoteReg()):
        self.noteReg = noteReg

    def register_note(self, node, note):
        if node not in self.noteReg.nodeList:
            self.noteReg.addNode(node)
        
        self.noteReg.addNote(SecretNote(-1, note.proof, node), node)
        return True

    def encoded_register_split(self, node, proof0, proof1, proof2):
        if node not in self.noteReg.nodeList:
            print('Fail to split the note: client not found')
            return False
        
        oldNote = SecretNote()
        for note in self.noteReg.currentNotes:
            if note.owner == node or note.proof == proof0:
                oldNote = note
                break
            
        if oldNote.proof != proof0:
            print('Fail to split the note: note not found')
            return False
        
        if common.rescue_verify(proof0, proof1, proof2):
            self.noteReg.deleteNote(oldNote, node)
            self.noteReg.addNote(SecretNote(-1, proof1, node), node)
            self.noteReg.addNote(SecretNote(-1, proof2, node), node)

            print('note split in registry')
            return True

        return False

    def client_submit_tx(self, sender, receiver, note):

        tempNote = SecretNote(-1, note.proof, sender)
        if sender not in self.noteReg.nodeList or tempNote not in self.noteReg.currentNotes:
            print('Tx failed: client or note not found')
            return False
        
        if receiver not in self.noteReg.nodeList:
            self.noteReg.addNode(receiver)

        self.noteReg.deleteNote(tempNote, sender)
        self.noteReg.addNote(SecretNote(-1, note.proof, receiver), receiver)

        return True
