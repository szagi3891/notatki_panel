import { MobxValueConnect } from 'src/common/MobxValueConnect';

const startCounter = (mobxValue: MobxValueConnect<void, number, NodeJS.Timer>): NodeJS.Timer => {
    return setInterval(() => {
        mobxValue.setValue(mobxValue.value + 1);
    }, 1000);
};

const stopCounter = (timer: NodeJS.Timer) => {
    clearInterval(timer);
};

export class CounterState {
    readonly counter: MobxValueConnect<void, number, NodeJS.Timer>;

    constructor() {
        this.counter = new MobxValueConnect<void, number, NodeJS.Timer>(undefined, 0, startCounter, stopCounter);
    }
}