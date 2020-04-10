#!/usr/bin/env python3
# -*- coding: utf-8 -*-

REPO = "/home/user/"

with open(REPO+"dico_original.txt", mode="r") as source:
    with open(REPO+"dico.txt", mode="w+") as destination:
        contenu = source.read()
        contenu = contenu.replace("\r\n", " ")
        contenu = contenu.replace("\n", " ")
        destination.write(contenu)
