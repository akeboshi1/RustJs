export function greet(str) {
    console.log(`greet ${str}`);
    return str;
}

export class Greet {
    constructor() {
        this._number = 42;
    }

    get number() {
        return this._number;
    }

    set number(n) {
        return this._number = n;
    }

    render() {
        return `My number is: ${this.number}`;
    }
}