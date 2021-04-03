(ns bookstore.core-spec
  (:require-macros [speclj.core :refer [describe it should= should should-not]])
  (:require [speclj.core]
            [reagent.core :as reagent :refer [atom]]
            [reagent.dom :as rdom]
            [bookstore.core :as rc]))


(def isClient (not (nil? (try (.-document js/window)
                              (catch js/Object e nil)))))

(def rflush reagent/flush)

(defn add-test-div [name]
  (let [doc     js/document
        body    (.-body js/document)
        div     (.createElement doc "div")]
    (.appendChild body div)
    div))

(defn with-mounted-component [comp f]
  (when isClient
    (let [div (add-test-div "_testreagent")]
      (let [comp (rdom/render comp div #(f comp div))]
        (rdom/unmount-component-at-node div)
        (rflush)
        (.removeChild (.-body js/document) div)))))


(defn found-in [re div]
  (let [res (.-innerHTML div)]
    (if (re-find re res)
      true
      (do (println "Not found: " res)
          false))))


(describe "test home"
  (it "contains 'Welcome to' in home page"
      (with-mounted-component (rc/home-page)
        (fn [c div]
          (should (found-in #"Welcome to" div))))))
