;;; cat.sexp — the smallest real CLI app: read the file named by the
;;; first argument and write it to stdout. Exercises the whole
;;; request/response effect protocol (GetArgs → ReadFile → Write → Exit).
;;;
;;;   check cli examples/cli/cat.sexp -- <file>
;;;
;;; The app is a state machine over `Phase`; each tick consumes the
;;; Event the driver fed back and emits the next request Action.

(type Action
  (GetArgs)
  (ReadFile (List Int))
  (Write    (List Int))
  (Exit     Int))

(type Event
  (Started)
  (Args   (List (List Int)))
  (FileOk (List Int))
  (FileErr)
  (Wrote))

(type (Step S A) (Step S A))

(type Phase (PStart) (PRead) (PEnd))

(fn cat ((s Phase) (e Event)) (Step Phase Action)
  (match s
    (PStart
      (match e
        (Started        (Step PStart (GetArgs)))          ; kick: ask for argv
        ((Args files)
          (match files
            (Nil             (Step PEnd  (Exit 1)))        ; no file argument
            ((Cons f rest)   (Step PRead (ReadFile f)))))  ; read the first arg
        (_              (Step PEnd  (Exit 2)))))
    (PRead
      (match e
        ((FileOk bytes) (Step PEnd  (Write bytes)))        ; got it → emit it
        (FileErr        (Step PEnd  (Exit 1)))
        (_              (Step PEnd  (Exit 2)))))
    (PEnd
      (match e
        (Wrote          (Step PEnd  (Exit 0)))             ; written → done
        (_              (Step PEnd  (Exit 0)))))))

(cli
  (state  Phase)
  (init   PStart)
  (update cat))
