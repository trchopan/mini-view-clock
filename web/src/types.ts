export enum View {
    Clock = 'Clock',
    Note = 'Note',
    About = 'About',
    HandGesture = 'HandGesture',
}

export enum SessionType {
    Work = 'Work',
    Short = 'Short',
    Long = 'Long',
}

export const SessionColors: Record<SessionType, string> = {
    [SessionType.Work]: '#e67e22', // carrot orange
    [SessionType.Short]: '#2ecc71', // green
    [SessionType.Long]: '#3498db', // blue
}
