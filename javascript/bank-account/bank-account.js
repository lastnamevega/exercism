export class BankAccount {
  constructor() {
    this._open = false;
  }

  open() {
    if (this._open) {
      throw new ValueError();
    }

    this._open = true;
    this._balance = 0;
  }

  close() {
    if (!this._open) {
      throw new ValueError();
    }

    this._open = false;
  }

  updateBalance(amount, negative = false) {
    let intAmount = parseInt(amount);

    if (!this._open || intAmount <= 0) {
      throw new ValueError();
    }

    if (negative.toString() === 'true') {
      if (intAmount > this._balance) {
        throw new ValueError();
      }

      intAmount *= -1;
    }

    this._balance += intAmount;
  }

  deposit(amount) {
    this.updateBalance(amount);
  }

  withdraw(amount) {
    this.updateBalance(amount, true);
  }

  get balance() {
    if (!this._open) {
      throw new ValueError();
    }

    return this._balance;
  }
}

export class ValueError extends Error {
  constructor() {
    super('Bank account error');
  }
}
