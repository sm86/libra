from smart_contract import SmartContract 
from client import Node

def main():
    alice = Node('AlicePub', 'AlicePrv')
    bob = Node('BobPub', 'BobPrv')
    sc = SmartContract()

    alice.make_note(sc, 10)
    alice.submit_secret_note_tx(sc, bob.pubkey, alice.notes[0], 2)

    print('Alice\'s balance and notes: ', alice.balance, str(alice.notes))
    # print(bob.balance, str(bob.notes))
    print('Notes in the registry: ', sc.noteReg.currentNotes)

    return 0

if __name__ == "__main__":
    main()