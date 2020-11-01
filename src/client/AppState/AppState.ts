import { makeAutoObservable } from "mobx";

export class AppState {

    counter: number = 0;

    constructor() {
        makeAutoObservable(this);
    }
    
    inc = () => {
        this.counter++;
    }
}