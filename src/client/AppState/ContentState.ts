import { MobxMapAutoNew } from '../../common/MobxMapAutoNew';
import { MobxValueConnect } from '../../common/MobxValueConnect';

type ContentType = {
    type: 'file',
    lastWrite: number,
    content: string,
} | {
    type: 'dir',
    list: Array<string>,
};

type PathValueType = MobxValueConnect<string, ContentType | null, NodeJS.Timer>;

const startCounter = (_mobxValue: PathValueType): NodeJS.Timer => {
    return setInterval(() => {

        //mobxValue.setValue(mobxValue.value + 1);

        //Odpal pobieranie zawartsci tej ściezki

        /*
            odczytaj typ tego co jest pod tą ściezka

                jesli to katalog
                    odczytaj liste plikow i koniec
                
                jesli to plik
                    odczytaj czas ostatniego zapisu
                    jesli juz mamy przeczytana zawartosc dla tej daty to koniec

                    jesli to jest inna zawartosc, to przeczytaj ponownie tresc
                    zapisz czas ostatniego zapisu
                    zapisz zawartosc tego pliku w zmiennej obserwowalnej
        */

        //....

    }, 1000);
};

const stopCounter = (timer: NodeJS.Timer) => {
    clearInterval(timer);
};

export class ContentState {
    readonly data: MobxMapAutoNew<string, PathValueType>;

    constructor() {
        this.data = new MobxMapAutoNew((path: string): PathValueType => {
            return new MobxValueConnect<string, ContentType | null, NodeJS.Timer>(path, null, startCounter, stopCounter);
        });
    }
}