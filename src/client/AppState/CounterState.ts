import { makeAutoObservable, onBecomeObserved, onBecomeUnobserved } from 'mobx';

export class CounterState {

    private timer: NodeJS.Timer | null = null;
    counter: number = 0;

    constructor() {
        makeAutoObservable(this);

        onBecomeObserved(this, 'counter', () => {
            console.info('timer start');
            this.timer = setInterval(() => {
                this.counter++;
            }, 1000);
        });

        onBecomeUnobserved(this, 'counter', () => {
            console.info('timer stop');
            if (this.timer !== null) {
                clearInterval(this.timer);
                this.timer = null;
            }
        })
    }
}