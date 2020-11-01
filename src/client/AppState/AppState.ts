//import { makeAutoObservable } from "mobx";
import { CounterState } from "./CounterState";

export class AppState {

    readonly counter: CounterState;

    constructor() {
        this.counter = new CounterState();
        // makeAutoObservable(this);
    }
    
    // inc = () => {
    //     this.counter++;
    // }
}