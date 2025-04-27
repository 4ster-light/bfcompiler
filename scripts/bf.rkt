#lang racket

(define max-prog-size 30000)

(define-struct memory-out-of-bounds ())
(define-struct unmatched-bracket ())

(define (check-bounds ptr array)
  (when (or (< ptr 0) (>= ptr (vector-length array)))
    (raise (memory-out-of-bounds))))

(define (find-matching-brackets code)
  (let* ([len (string-length code)]
         [brackets (make-vector len -1)]
         [stack '()])
    (for ([i (in-range len)])
      (case (string-ref code i)
        [(#\[) (set! stack (cons i stack))]
        [(#\]) (if (null? stack)
                   (raise (unmatched-bracket))
                   (let ([open-pos (car stack)])
                     (vector-set! brackets open-pos i)
                     (vector-set! brackets i open-pos)
                     (set! stack (cdr stack))))]))
    (unless (null? stack) (raise (unmatched-bracket)))
    brackets))

(define (interpret-bf code)
  (let ([array (make-vector max-prog-size 0)]
        [brackets (find-matching-brackets code)])
    (let loop ([ptr 0] [code-ptr 0])
      (when (< code-ptr (string-length code))
        (check-bounds ptr array)
        (case (string-ref code code-ptr)
          [(#\+) (vector-set! array ptr (add1 (vector-ref array ptr)))
                 (loop ptr (add1 code-ptr))]
          [(#\-) (vector-set! array ptr (sub1 (vector-ref array ptr)))
                 (loop ptr (add1 code-ptr))]
          [(#\<) (loop (sub1 ptr) (add1 code-ptr))]
          [(#\>) (loop (add1 ptr) (add1 code-ptr))]
          [(#\,) (vector-set! array ptr (char->integer (read-char)))
                 (loop ptr (add1 code-ptr))]
          [(#\.) (write-char (integer->char (vector-ref array ptr)))
                 (flush-output)
                 (loop ptr (add1 code-ptr))]
          [(#\[) (if (zero? (vector-ref array ptr))
                     (loop ptr (add1 (vector-ref brackets code-ptr)))
                     (loop ptr (add1 code-ptr)))]
          [(#\]) (if (not (zero? (vector-ref array ptr)))
                     (loop ptr (vector-ref brackets code-ptr))
                     (loop ptr (add1 code-ptr)))]
          [else (loop ptr (add1 code-ptr))])))))

(module+ main
  (command-line
   #:program "bf-interpreter"
   #:args (filename)
   (with-handlers ([exn:fail:filesystem? (λ (e) (eprintf "Error: ~a\n" (exn-message e)))]
                   [memory-out-of-bounds? (λ (_) (eprintf "Error: Memory access out of bounds\n"))]
                   [unmatched-bracket? (λ (_) (eprintf "Error: Unmatched bracket\n"))])
     (interpret-bf (file->string filename)))))
