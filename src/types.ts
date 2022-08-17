export interface Person {
    id: number;
    name: string;
    summary: string;
}

export interface Note {
    id: number;
    created_on: string;
    text: string;

    person_id: number;
}
