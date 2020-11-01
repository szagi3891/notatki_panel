import { makeAutoObservable } from "mobx";
import { ContentState } from "./ContentState";
import { CounterState } from "./CounterState";
import { CurrentPath } from "./CurrentPath";

export class AppState {
    private readonly counter: CounterState;
    readonly currentPath: CurrentPath;

    constructor() {
        makeAutoObservable(this); //for computed

        this.counter = new CounterState();
        this.currentPath = new CurrentPath('', new ContentState())
    }

    get counterValue(): number {
        return this.counter.counter.value;
    }
}
