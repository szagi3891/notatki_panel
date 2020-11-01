import { observable, makeObservable, onBecomeObserved, onBecomeUnobserved } from 'mobx';

export class MobxValueConnect<K, T, S> {
    readonly id: K;
    value: T;
    private subscription: S | null = null;
    private onConnect: (mobxValue: MobxValueConnect<K, T, S>) => S;
    private onDisconnect: (sub: S) => void;

    constructor(id: K, value: T, onConnect: (mobxValue: MobxValueConnect<K, T, S>) => S, onDisconnect: (sub: S) => void) {
        this.id = id;
        this.value = value;
        this.onConnect = onConnect;
        this.onDisconnect = onDisconnect;

        makeObservable(this, {
            value: observable,
        });

        onBecomeObserved(this, 'value', () => {
            console.info('value start observe', this.id);

            if (this.subscription !== null) {
                throw Error('onBecomeObserved - Nieprawidłowy stan')
            }

            this.subscription = this.onConnect(this);
        });

        onBecomeUnobserved(this, 'value', () => {
            console.info('value stop observe', this.id);

            if (this.subscription === null) {
                throw Error('onBecomeUnobserved - Nieprawidłowy stan')
            }

            this.onDisconnect(this.subscription);
            this.subscription = null;
        });
    }

    setValue(value: T) {
        this.value = value;
    }
}
