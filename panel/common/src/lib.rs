use serde::{Deserialize, Serialize};


pub type TimestampType = u128;
pub type DataNodeIdType = u64;


#[derive(Deserialize, Serialize)]
pub enum DataNode {
    File {
        id: DataNodeIdType,
        title: String,
        content: String,
    },
    Dir {
        id: DataNodeIdType,
        title: String,
        child: Vec<DataNodeIdType>,         //rozjazdowka na kolejne dzieci z trescia
    }
}

//Ta struktura będzie latać na handlerach http ...
#[derive(Deserialize, Serialize)]
pub struct DataPost {
    pub timestamp: TimestampType,
    pub node: DataNode,
}

/*

domyślny korzeń od którego się zaczyna wszystko
main ---> 1


baza danych typu klucz wartość (git jako nośnik danych)

collections /
    notatki /
        p001            --> element 1
        d023/p032       --> element 23032

    ta warstwa zapewnia tylko zapis i odczyt tych elementów

    save
        fajnie byłoby mieć jakiś atomowy zapis ...




    element:
        ostatni timestamp zmiany ...
        data: ....      dane binarne

    2018-01-26T18:30:09.453Z

    Z - moze byc markerem oznaczajacym koniec daty.
    ALbo, mozna wybrac inna literke

    
    zapis mozliwy bylby tylko pod warunkiem ze w request podalismy timestamp aktualnej tresci
    nowy timestamp musi byc wiekszy od tego ktory jest obecnie

    




    GET /get/:id        - pobranie elementu
    POST /save/:id       - zapisanie nowego elementu
        {
            data: dane do zapisu
            timesamp - data ostatniego zapisu, potrzebne w celu potwierdzenia czy ta zmiana nadal jest świeza
        }
*/