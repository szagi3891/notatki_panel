import { observable, makeObservable } from 'mobx';

export class MobxValue<T> {
    value: T;

    constructor(value: T) {
        this.value = value;

        makeObservable(this, {
            value: observable,
        });
    }

    setValue(value: T) {
        this.value = value;
    }
}