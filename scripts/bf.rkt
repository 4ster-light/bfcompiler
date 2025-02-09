#lang racket

(define MAX_PROG_SIZE 30000)

(define (check-bounds ptr array)
  (when (>= ptr (vector-length array))
    (error 'check-bounds "Memory access out of bounds")))

(define (interpret-bf bf-code)
  (let* ((array (make-vector MAX_PROG_SIZE 0))
         (ptr (box 0))
         (code-ptr (box 0))
         (loop-stack '()))
    (let loop ()
      (check-bounds (unbox ptr) array)
      (when (< (unbox code-ptr) (string-length bf-code))
        (let ((char (string-ref bf-code (unbox code-ptr))))
          (case char
            [(#\+) (vector-set! array (unbox ptr) (+ (vector-ref array (unbox ptr)) 1))]
            [(#\-) (vector-set! array (unbox ptr) (- (vector-ref array (unbox ptr)) 1))]
            [(#\<) (set-box! ptr (max 0 (- (unbox ptr) 1)))]
            [(#\>) (set-box! ptr (+ (unbox ptr) 1))]
            [(#\,) (let ((input (read-char)))
                     (unless (eof-object? input)
                       (vector-set! array (unbox ptr) (char->integer input))))]
            [(#\.) (write-char (integer->char (vector-ref array (unbox ptr))))]
            [(#\[) (if (= (vector-ref array (unbox ptr)) 0)
                       (let ((balance 1)
                             (depth 1))
                         (set-box! code-ptr (+ (unbox code-ptr) 1))
                         (let loop ()
                           (when (< (unbox code-ptr) (string-length bf-code))
                             (let ((current-char (string-ref bf-code (unbox code-ptr))))
                               (case current-char
                                 [(#\[) (set! balance (+ balance 1))]
                                 [(#\]) (set! balance (- balance 1))]
                                 [else (void)])
                               (set! depth balance)
                               (set-box! code-ptr (+ (unbox code-ptr) 1))
                               (unless (= depth 0) (loop))))))
                       (set! loop-stack (cons (unbox code-ptr) loop-stack)))]
            [(#\]) (if (= (vector-ref array (unbox ptr)) 0)
                       (set! loop-stack (cdr loop-stack))
                       (set-box! code-ptr (car loop-stack)))]
            [else (void)])
          (set-box! code-ptr (+ (unbox code-ptr) 1))
          (loop))))
    (void)))

(define (main file-path)
  (let* ((bf-code (with-input-from-file file-path port->string)))
    (interpret-bf bf-code)))

(module+ main
  (command-line
   #:program "bf"
   #:args (filename)
   (main filename)))
