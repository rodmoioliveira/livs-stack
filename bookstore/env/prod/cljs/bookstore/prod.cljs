(ns bookstore.prod
  (:require [bookstore.core :as core]))

;;ignore println statements in prod
(set! *print-fn* (fn [& _]))

(core/init!)
