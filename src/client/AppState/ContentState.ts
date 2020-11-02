import { MobxMapAutoNew } from 'src/common/MobxMapAutoNew';
import { MobxValueConnect } from 'src/common/MobxValueConnect';
import { apiGetPath, ApiGetPathResponseType } from '../api/apiGetPath';

type ContentType = {
    type: 'file',
    lastWrite: number,
    content: string,
} | {
    type: 'dir',
    list: Array<string>,
};

const convert = (data: ApiGetPathResponseType): ContentType | null => {
    if (data === null) {
        return null;
    }

    if (data.type === 'dir') {
        return data;
    }

    return {
        ...data,
        content: '',                            //TODO - temporary
    }
};

type PathValueType = MobxValueConnect<string, ContentType | null, NodeJS.Timer>;

const startCounter = (mobxValue: PathValueType): NodeJS.Timer => {
    return setInterval(() => {

        const path = mobxValue.id;

        apiGetPath(path).then((response) => {
            console.info('response', response);

            mobxValue.setValue(convert(response));
        }).catch((error) => {
            console.error(error);
        });

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

    }, 3000);
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

    getPath(path: string): ContentType | null {
        return this.data.get(path).value;
    }
}