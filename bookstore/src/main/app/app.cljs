(ns app.app
  (:require [reagent.core :as r]
            [reagent.dom :as rdom]))

(def click-count (r/atom 0))

(defn counting-component []
  [:p
   "The atom " [:code "click-count"] " has value: "
   @click-count ". "
   [:input {:type "button"
            :value "Click me!"
            :on-click #(swap! click-count inc)}]])

(defn mount-root
  "Initialize app"
  []
  (rdom/render [counting-component] (.getElementById js/document "root")))

(defn init! []
  (mount-root)
  (js/console.log "Hello World"))

