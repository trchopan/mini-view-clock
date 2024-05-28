export enum View {
  Clock = 'Clock',
  Note = 'Note',
  About = 'About',
  HandGesture = 'HandGesture',
}

const throwError = (e: any) => {
  throw new Error(e)
}

export const strToView = (s: string) => {
  return (
    {
      Clock: View.Clock,
      Note: View.Note,
    }[s] || throwError('cannot convert string to View')
  )
}

export interface NoteHeader {
  emoji: string | null
  content: string
}
